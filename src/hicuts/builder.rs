use alloc::vec::Vec;
use alloc::boxed::Box;
use crate::rule::{Rule, Range};
use crate::cutsplit::tree::Dimension;
use crate::hicuts::tree::Node;


pub struct Builder {
    pub leaf_threshold: usize,
    pub max_depth: usize,
    pub binth: usize, // Max cuts multiplier or similar tuning param
    pub spfac: usize, // Space factor max expansion
}

impl Builder {
    pub fn new(leaf_threshold: usize, max_depth: usize) -> Self {
        Self { leaf_threshold, max_depth, binth: 8, spfac: 4 }
    }

    pub fn build(&self, rules: &[Rule]) -> Node {
        // Initial region: Full 5-tuple space
        // We track the current range for each dimension to calculate cuts
        let ranges = [
            (Dimension::SrcIp, 0, u32::MAX),
            (Dimension::DstIp, 0, u32::MAX),
            (Dimension::SrcPort, 0, 65535),
            (Dimension::DstPort, 0, 65535),
            (Dimension::Proto, 0, 255),
        ];
        
        self.build_recursive(rules, 0, &ranges)
    }

    fn build_recursive(&self, rules: &[Rule], depth: usize, ranges: &[(Dimension, u32, u32)]) -> Node {
        if rules.len() <= self.leaf_threshold || depth >= self.max_depth {
            return Node::Leaf { rules: rules.to_vec() };
        }

        // Heuristic: Select dimension and number of cuts
        let (best_dim, num_cuts) = self.select_dimension_and_cuts(rules, ranges);

        if num_cuts <= 1 {
            // Cannot cut effectively
             return Node::Leaf { rules: rules.to_vec() };
        }

        // Create children
        let range_info = ranges.iter().find(|(d, _, _)| *d == best_dim).unwrap();
        let (dim, min_val, max_val) = *range_info;
        
        let range_size = max_val as u64 - min_val as u64 + 1;
        let step = (range_size / num_cuts as u64) as u32; // Integer division, last bin might be larger/smaller slightly?
        // To be safe in coverage, careful with step size. 
        // Simplification: Divide linearly.
        
        let mut children = Vec::with_capacity(num_cuts as usize);
        
        for i in 0..num_cuts {
            let cut_min = min_val + i * step;
            let cut_max = if i == num_cuts - 1 { max_val } else { min_val + (i + 1) * step - 1 };
            
            // Filter rules
            let mut child_rules = Vec::new();
            for rule in rules {
                if self.rule_overlaps(rule, dim, cut_min, cut_max) {
                    child_rules.push(rule.clone());
                }
            }
            
            // Recurse
            let mut new_ranges = ranges.to_vec();
            for r in &mut new_ranges {
                if r.0 == dim {
                    *r = (dim, cut_min, cut_max);
                    break;
                }
            }
            
            children.push(Box::new(self.build_recursive(&child_rules, depth + 1, &new_ranges)));
        }

        Node::Internal {
            dimension: dim,
            start: min_val,
            step,
            num_cuts: num_cuts as u32,
            children,
        }
    }

    fn select_dimension_and_cuts(&self, rules: &[Rule], ranges: &[(Dimension, u32, u32)]) -> (Dimension, u32) {
        let mut best_dim = Dimension::SrcIp;
        let mut best_cut_count = 1;
        let mut min_max_rules = usize::MAX;

        for &(dim, min_val, max_val) in ranges {
            // Can't cut if range is singular
            if min_val >= max_val { continue; }

            // Try cuts: 2, 4, 8, ... up to 16? Simplified HiCuts
            for &cuts in &[2, 4, 8, 16] { 
                 let range_len = max_val as u64 - min_val as u64 + 1;
                 if range_len < cuts as u64 { continue; }

                 let step = (range_len / cuts as u64) as u32;
                 let mut max_rules_in_bin = 0;
                 let mut _total_rules = 0;

                 for i in 0..cuts {
                    let c_min = min_val + i * step;
                    let c_max = if i == cuts - 1 { max_val } else { min_val + (i + 1) * step - 1 };
                    
                    let mut bin_count = 0;
                    for rule in rules {
                        if self.rule_overlaps(rule, dim, c_min, c_max) {
                            bin_count += 1;
                        }
                    }
                    if bin_count > max_rules_in_bin { max_rules_in_bin = bin_count; }

                 }

                 // Cost function: Minimize max bucket size + penalty for duplication
                 // Simplified: Just minimize max bucket size for now, ensure "progress".
                 if max_rules_in_bin < min_max_rules && max_rules_in_bin < rules.len() {
                     min_max_rules = max_rules_in_bin;
                     best_dim = dim;
                     best_cut_count = cuts;
                 }
            }
        }
        
        (best_dim, best_cut_count)
    }

    fn rule_overlaps(&self, rule: &Rule, dim: Dimension, min_val: u32, max_val: u32) -> bool {
        let range = match dim {
            Dimension::SrcIp => rule.src_ip,
            Dimension::DstIp => rule.dst_ip,
            Dimension::SrcPort => Range::new(rule.src_port.min as u32, rule.src_port.max as u32),
            Dimension::DstPort => Range::new(rule.dst_port.min as u32, rule.dst_port.max as u32),
            Dimension::Proto => Range::new(rule.proto.min as u32, rule.proto.max as u32),
        };
        
        // Range overlap: rule.min <= region.max && rule.max >= region.min
        range.min <= max_val && range.max >= min_val
    }
}
