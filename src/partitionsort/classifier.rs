//! PartitionSort Classifier Implementation
//!
//! Based on the paper:
//! "A Sorted-Partitioning Approach to Fast and Scalable Dynamic Packet Classification"
//! Yingchareonthawornchai, et al. (IEEE Transactions on Networking 2018)
//! <https://ieeexplore.ieee.org/document/7774710>

use alloc::vec::Vec;
use crate::classifier::Classifier;
use crate::packet::FiveTuple;
use crate::rule::{Rule, Action};
use crate::partitionsort::tree::{IntervalTree, Node};

pub struct PartitionSortClassifier {
    // For now, simpler version: Just multiple IntervalTrees (partitions) searched linearly?
    // Or just one best one?
    // If we want to implement the "Partition" part:
    // We split rules into subsets. Each subset has its own IntervalTree.
    trees: Vec<IntervalTree>,
}

impl PartitionSortClassifier {
    // Heuristic: Evaluate a dimension. Returns a score (lower is better).
    // Score = Max bucket size in the tree?
    fn evaluate_dimension(rules: &[Rule], dim: usize) -> usize {
        // Build a temporary tree (or just simulate) to find max collision depth
        // Simulation is cheaper.
        // But `IntervalTree::build` is fast enough for setup.
        let tree = IntervalTree::build(rules.to_vec(), dim);
        Self::get_max_bucket_size(&tree)
    }

    fn get_max_bucket_size(tree: &IntervalTree) -> usize {
        tree.root.as_ref().map_or(0, |n| Self::max_bucket_recursive(n))
    }

    fn max_bucket_recursive(node: &Node) -> usize {
        let my_size = node.rules.len();
        let left_max = node.left.as_ref().map_or(0, |n| Self::max_bucket_recursive(n));
        let right_max = node.right.as_ref().map_or(0, |n| Self::max_bucket_recursive(n));
        my_size.max(left_max).max(right_max)
    }
}

impl Classifier for PartitionSortClassifier {
    fn build(rules: &[Rule]) -> Self {
        if rules.is_empty() {
            return Self { trees: Vec::new() };
        }

        // Implementation of a greedy logic:
        // 1. Try to put ALL rules into one tree on best dim.
        // 2. If max bucket size is too high, implies "bad sortability" for some rules.
        // 3. (Partitioning Step - TODO for V2): Extract "bad" rules and put in next partition.
        // For V1, we just pick the Single Best Dimension.
        // This effectively makes it a "1D Layout Optimized" classifier.
        
        // Check 5 dims
        let mut best_dim = 0;
        let mut min_max_bucket = usize::MAX;

        for dim in 0..5 {
            let score = Self::evaluate_dimension(rules, dim);
             // Prefer Src/Dst IP (0,1) over Ports (2,3) if scores tie, generally more entropy
            if score < min_max_bucket {
                min_max_bucket = score;
                best_dim = dim;
            }
        }

        let best_tree = IntervalTree::build(rules.to_vec(), best_dim);
        
        Self { trees: alloc::vec![best_tree] }
    }

    fn classify(&self, packet: &FiveTuple) -> Option<Action> {
        let mut best_match: Option<&Rule> = None;

        for tree in &self.trees {
            // Extract value for this tree's dimension
            let val = match tree.field_idx {
                0 => packet.src_ip,
                1 => packet.dst_ip,
                2 => packet.src_port as u32,
                3 => packet.dst_port as u32,
                4 => packet.proto as u32,
                _ => 0,
            };

            if let Some(rule) = tree.classify_packet(packet, val) {
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
