//! Tuple Space Search (TSS) and TupleMerge Classifier Implementation
//!
//! TSS based on:
//! "Packet Classification on Multiple Fields"
//! V. Srinivasan, et al. (SIGCOMM 1999)
//!
//! TupleMerge optimization based on:
//! "TupleMerge: Building Online Packet Classifiers by Omitting Bits"
//! James Daly, et al. (IEEE Transactions on Networking 2019)
//! <https://ieeexplore.ieee.org/document/8038296>

use alloc::vec::Vec;
use hashbrown::HashMap;
use crate::classifier::Classifier;
use crate::packet::FiveTuple;
use crate::rule::{Rule, Action};
use crate::tss::utils::{range_to_prefixes_u32, range_to_prefixes_u16, range_to_prefixes_u8};

/// A Tuple represents the prefix lengths for the 5 fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Tuple {
    src_ip_len: u32,
    dst_ip_len: u32,
    src_port_len: u32,
    dst_port_len: u32,
    proto_len: u32,
}

impl Tuple {
    /// Check if `self` is a subset of `other` (meaning `self` masks fewer or equal bits).
    /// If `self` is a subset, a rule from `other` can be stored in `self`'s table.
    fn is_subset_of(&self, other: &Tuple) -> bool {
        self.src_ip_len <= other.src_ip_len &&
        self.dst_ip_len <= other.dst_ip_len &&
        self.src_port_len <= other.src_port_len &&
        self.dst_port_len <= other.dst_port_len &&
        self.proto_len <= other.proto_len
    }

    /// Calculate total bit difference between two tuples.
    fn bit_difference(&self, other: &Tuple) -> u32 {
        (other.src_ip_len - self.src_ip_len) +
        (other.dst_ip_len - self.dst_ip_len) +
        (other.src_port_len - self.src_port_len) +
        (other.dst_port_len - self.dst_port_len) +
        (other.proto_len - self.proto_len)
    }
}

/// Key for the Hash Map: The masked values of the fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct TupleKey {
    src_ip: u32,
    dst_ip: u32,
    src_port: u16,
    dst_port: u16,
    proto: u8,
}

impl TupleKey {
    fn new(packet: &FiveTuple, tuple: &Tuple) -> Self {
        Self {
            src_ip: Self::mask_u32(packet.src_ip, tuple.src_ip_len),
            dst_ip: Self::mask_u32(packet.dst_ip, tuple.dst_ip_len),
            src_port: Self::mask_u16(packet.src_port, tuple.src_port_len),
            dst_port: Self::mask_u16(packet.dst_port, tuple.dst_port_len),
            proto: Self::mask_u8(packet.proto, tuple.proto_len),
        }
    }
    
    // Create a key from values but masked by the Tuple
    fn from_values(src_ip: u32, dst_ip: u32, src_port: u16, dst_port: u16, proto: u8, tuple: &Tuple) -> Self {
         Self {
            src_ip: Self::mask_u32(src_ip, tuple.src_ip_len),
            dst_ip: Self::mask_u32(dst_ip, tuple.dst_ip_len),
            src_port: Self::mask_u16(src_port, tuple.src_port_len),
            dst_port: Self::mask_u16(dst_port, tuple.dst_port_len),
            proto: Self::mask_u8(proto, tuple.proto_len),
        }
    }

    fn mask_u32(val: u32, len: u32) -> u32 {
        if len == 0 { return 0; }
        if len >= 32 { return val; }
        val & (!0u32 << (32 - len))
    }

    fn mask_u16(val: u16, len: u32) -> u16 {
        if len == 0 { return 0; }
        if len >= 16 { return val; }
        val & (!0u16 << (16 - len))
    }

    fn mask_u8(val: u8, len: u32) -> u8 {
        if len == 0 { return 0; }
        if len >= 8 { return val; }
        val & (!0u8 << (8 - len))
    }
}

/// Tuple Space Classifier
pub struct TSSClassifier {
    /// List of tuples and their corresponding hash tables.
    /// To support multiple rules per key (collisions due to merging), the value is a Vec<Rule>.
    tables: HashMap<Tuple, HashMap<TupleKey, Vec<Rule>>>,
    _marker: (),
}

impl TSSClassifier {
    /// Cartesian product of prefixes
    fn expand_rule(rule: &Rule) -> Vec<(Tuple, u32, u32, u16, u16, u8)> {
        let src_prefixes = range_to_prefixes_u32(rule.src_ip.min, rule.src_ip.max, 32);
        let dst_prefixes = range_to_prefixes_u32(rule.dst_ip.min, rule.dst_ip.max, 32);
        let sp_prefixes  = range_to_prefixes_u16(rule.src_port.min, rule.src_port.max);
        let dp_prefixes  = range_to_prefixes_u16(rule.dst_port.min, rule.dst_port.max);
        let proto_prefixes = range_to_prefixes_u8(rule.proto.min, rule.proto.max);

        let mut expanded = Vec::new();

        for s in &src_prefixes {
            for d in &dst_prefixes {
                for sp in &sp_prefixes {
                    for dp in &dp_prefixes {
                        for pr in &proto_prefixes {
                            let tuple = Tuple {
                                src_ip_len: s.len,
                                dst_ip_len: d.len,
                                src_port_len: sp.len,
                                dst_port_len: dp.len,
                                proto_len: pr.len,
                            };
                            // We return raw values here, correct key depends on the *Table* tuple derived later
                            expanded.push((tuple, s.value, d.value, sp.value, dp.value, pr.value));
                        }
                    }
                }
            }
        }
        expanded
    }
}

impl Classifier for TSSClassifier {
    fn build(rules: &[Rule]) -> Self {
        let mut tables: HashMap<Tuple, HashMap<TupleKey, Vec<Rule>>> = HashMap::new();
        
        // Configuration for TupleMerge
        // Max bits difference allowed to merge. Higher = fewer tables, more collisions.
        // A full 5-tuple has 96+ bits effectively.
        // Let's try a conservative limit first to group "very close" ranges.
        const MAX_MERGE_BITS: u32 = 12; 

        for rule in rules {
            let expanded_parts = Self::expand_rule(rule);
            
            for (rule_tuple, sip, dip, sport, dport, proto) in expanded_parts {
                
                // TupleMerge Strategy: Find best existing table
                let mut best_table_tuple: Option<Tuple> = None;
                let mut min_diff = u32::MAX;
                
                // collect keys to avoid borrow overlap if needed, or just iterate
                for existing_tuple in tables.keys() {
                    if existing_tuple.is_subset_of(&rule_tuple) {
                        let diff = existing_tuple.bit_difference(&rule_tuple);
                        if diff < min_diff && diff <= MAX_MERGE_BITS {
                            min_diff = diff;
                            best_table_tuple = Some(*existing_tuple);
                        }
                    }
                }
                
                // If no good match found, we use the rule's tuple as a new table
                let target_tuple = best_table_tuple.unwrap_or(rule_tuple);
                
                let table = tables.entry(target_tuple).or_insert_with(HashMap::new);
                
                // Generate key using the TARGET tuple (masking based on table definition)
                let key = TupleKey::from_values(sip, dip, sport, dport, proto, &target_tuple);
                
                let bucket = table.entry(key).or_insert_with(Vec::new);
                // Insert rule if better priority or just append? 
                // Since we have collisions, we MUST append and scan all.
                // Optim: keep sorted by priority?
                bucket.push(rule.clone());
                // Sort bucket by priority (ascending value = higher priority)
                bucket.sort_by_key(|r| r.priority);
            }
        }

        Self { tables, _marker: () }
    }

    fn classify(&self, packet: &FiveTuple) -> Option<Action> {
        let mut best_match: Option<&Rule> = None;

        for (tuple, table) in &self.tables {
            let key = TupleKey::new(packet, tuple);
            if let Some(bucket) = table.get(&key) {
                // Determine if we found a match in this bucket
                for rule in bucket {
                    // Start with high priority check
                    // If we already have a match with priority P, and this rule has priority > P (value < P), we check.
                    // If rule priority < best_match priority (value > best), we can stop if sorted? 
                    // No, because we iterate tables in arbitrary order. We must scan all tables.
                    
                    // Optimization: If rule.priority >= best_match.priority (value >=), we can skip checking?
                    // Only if we are sure this rule matches. But we aren't.
                    // We need to check exact match first.
                    
                    if let Some(best) = best_match {
                         if rule.priority >= best.priority {
                             // This rule is lower or equal priority than what we have. 
                             // Since bucket is sorted, subsequent rules are also worse.
                             break; 
                         }
                    }
                    
                    if rule.matches(packet) {
                        match best_match {
                            None => best_match = Some(rule),
                            Some(best) => {
                                if rule.priority < best.priority {
                                    best_match = Some(rule);
                                }
                            }
                        }
                         // Since bucket is sorted, and we found a match, any subsequent match in *this* bucket 
                         // will be lower priority. So we can stop this bucket scan.
                        break;
                    }
                }
            }
        }

        best_match.map(|r| r.action)
    }
}
