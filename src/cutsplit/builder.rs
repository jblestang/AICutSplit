use crate::cutsplit::tree::{Dimension, Node};
use crate::rule::{Range, Rule};
use alloc::boxed::Box;
use alloc::vec::Vec;

/// Builder for the CutSplit decision tree.
///
/// Implements the logic to construct the tree by recursively partitioning the rule set.
/// It uses heuristics to choose the best dimension and value to cut.
pub struct Builder {
    /// Max rules in a leaf before just stopping (linear search).
    pub leaf_threshold: usize,
    /// Maximum depth of the tree to prevent excessive size/stack usage.
    pub max_depth: usize,
}

impl Builder {
    /// Create a new builder with specified thresholds.
    pub fn new(leaf_threshold: usize, max_depth: usize) -> Self {
        Self {
            leaf_threshold,
            max_depth,
        }
    }

    /// Build a decision tree from a set of rules.
    pub fn build(&self, rules: &[Rule]) -> Node {
        self.build_recursive(rules, 0)
    }

    /// Recursively build the tree.
    fn build_recursive(&self, rules: &[Rule], depth: usize) -> Node {
        // Base case: Few enough rules or max depth reached
        if rules.len() <= self.leaf_threshold || depth >= self.max_depth {
            return Node::Leaf {
                rules: rules.to_vec(),
            };
        }

        // Try to find a good cut
        if let Some((dim, val)) = self.find_best_cut(rules) {
            let (left_rules, right_rules) = self.partition_rules(rules, dim, val);

            // Heuristic to stop if split is ineffective (e.g., all rules go to one side)
            // But strict duplication might cause both sides to have many rules if they all overlap.
            // If both children satisfy base condition check? No, we recurse.

            // If we didn't reduce the rule set size in at least one branch effectively, or if we are just duplicating everything:
            // For now, accept the cut if it exists.

            Node::Internal {
                dimension: dim,
                cut_val: val,
                left: Box::new(self.build_recursive(&left_rules, depth + 1)),
                right: Box::new(self.build_recursive(&right_rules, depth + 1)),
            }
        } else {
            // No good cut found
            Node::Leaf {
                rules: rules.to_vec(),
            }
        }
    }

    fn find_best_cut(&self, rules: &[Rule]) -> Option<(Dimension, u32)> {
        // Simple heuristic: Try to cut on IP/Port dimensions.
        // We look for a median point of start/end points of ranges in these dimensions.

        let dimensions = [
            Dimension::SrcIp,
            Dimension::DstIp,
            Dimension::SrcPort,
            Dimension::DstPort,
        ];
        let mut best_score = -1.0;
        let mut best_cut = None;

        for &dim in &dimensions {
            // Collect all endpoints
            let mut points = Vec::new();
            for rule in rules {
                let range = self.get_range(rule, dim);
                points.push(range.min);
                points.push(range.max.saturating_add(1)); // Exclusive end
            }
            points.sort_unstable();
            points.dedup();

            // Try potential cut points (e.g. median)
            // For speed, just check median or a few sample points.
            let mid_idx = points.len() / 2;
            if mid_idx > 0 && mid_idx < points.len() {
                let val = points[mid_idx];
                // Calculate score: Balance + Duplication penalty
                // Score = 1 - abs(left_count - right_count)/total - duplication_factor
                let (l, r) = self.count_split(rules, dim, val);

                // Avoid useless cuts
                if l == rules.len() && r == rules.len() {
                    continue;
                }
                if l == 0 || r == 0 {
                    continue;
                } // Pure split not useful if it doesn't separate? Wait, if l=0, all in right.

                let duplication = (l + r) as f32 / rules.len() as f32;
                // We want minimizing duplication (closer to 1.0) and creating balance.
                // Let's use negative duplication as score component.
                let score = 1.0 / duplication;

                if score > best_score {
                    best_score = score;
                    best_cut = Some((dim, val));
                }
            }
        }

        best_cut
    }

    fn partition_rules(&self, rules: &[Rule], dim: Dimension, val: u32) -> (Vec<Rule>, Vec<Rule>) {
        let mut left = Vec::new();
        let mut right = Vec::new();

        for rule in rules {
            let range = self.get_range(rule, dim);

            // Left child: < val
            // Range overlaps left if min < val
            if range.min < val {
                left.push(rule.clone());
            }

            // Right child: >= val
            // Range overlaps right if max >= val
            if range.max >= val {
                right.push(rule.clone());
            }
        }
        (left, right)
    }

    fn count_split(&self, rules: &[Rule], dim: Dimension, val: u32) -> (usize, usize) {
        let mut l = 0;
        let mut r = 0;
        for rule in rules {
            let range = self.get_range(rule, dim);
            if range.min < val {
                l += 1;
            }
            if range.max >= val {
                r += 1;
            }
        }
        (l, r)
    }

    fn get_range(&self, rule: &Rule, dim: Dimension) -> Range<u32> {
        match dim {
            Dimension::SrcIp => rule.src_ip,
            Dimension::DstIp => rule.dst_ip,
            Dimension::SrcPort => Range::new(rule.src_port.min as u32, rule.src_port.max as u32),
            Dimension::DstPort => Range::new(rule.dst_port.min as u32, rule.dst_port.max as u32),
            Dimension::Proto => Range::new(rule.proto.min as u32, rule.proto.max as u32),
        }
    }
}
