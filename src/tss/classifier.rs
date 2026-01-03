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
    /// List of tuples and their corresponding hash tables
    tables: HashMap<Tuple, HashMap<TupleKey, Rule>>,
    _marker: (),
}

impl TSSClassifier {
    /// Cartesian product of prefixes
    fn expand_rule(rule: &Rule) -> Vec<(Tuple, TupleKey)> {
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
                            let key = TupleKey {
                                src_ip: s.value,
                                dst_ip: d.value,
                                src_port: sp.value,
                                dst_port: dp.value,
                                proto: pr.value,
                            };
                            expanded.push((tuple, key));
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
        let mut tables: HashMap<Tuple, HashMap<TupleKey, Rule>> = HashMap::new();

        for rule in rules {
            let expanded = Self::expand_rule(rule);
            for (tuple, key) in expanded {
                let table = tables.entry(tuple).or_insert_with(HashMap::new);
                
                // If key exists, keep the one with lower priority value (higher priority).
                match table.get_mut(&key) {
                    Some(existing) => {
                        if rule.priority < existing.priority {
                            *existing = rule.clone();
                        }
                    },
                    None => {
                        table.insert(key, rule.clone());
                    }
                }
            }
        }

        Self { tables, _marker: () }
    }

    fn classify(&self, packet: &FiveTuple) -> Option<Action> {
        let mut best_match: Option<&Rule> = None;

        for (tuple, table) in &self.tables {
            let key = TupleKey::new(packet, tuple);
            if let Some(rule) = table.get(&key) {
                match best_match {
                    None => best_match = Some(rule),
                    Some(best) => {
                        if rule.priority < best.priority {
                            best_match = Some(rule);
                        }
                    }
                }
            }
        }

        best_match.map(|r| r.action)
    }
}
