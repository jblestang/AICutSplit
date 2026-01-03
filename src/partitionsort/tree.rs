use crate::rule::{Range, Rule};
use alloc::boxed::Box;
use alloc::vec::Vec;

/// Node in the Interval Tree
#[derive(Debug, Clone)]
pub struct Node {
    pub center: u32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
    // Rules covering the center point, sorted by priority?
    // In a standard interval tree, we store intervals overlapping the center.
    // However, for packet classification, we might want to store the Rule ID or reference.
    // PartitionSort paper suggests storing rules in a data structure that supports fast stabbing queries.
    // A simple list of rules at the node is fine for now, we iterate them.
    pub rules: Vec<Rule>,
}

impl Node {
    pub fn new(center: u32, rules: Vec<Rule>) -> Self {
        Self {
            center,
            left: None,
            right: None,
            rules,
        }
    }
}

/// A 1-Dimensional Interval Tree for a specific field Dimension.
#[derive(Debug, Clone)]
pub struct IntervalTree {
    pub root: Option<Box<Node>>,
    pub field_idx: usize, // 0=SrcIP, 1=DstIP, 2=SrcPort, 3=DstPort, 4=Proto
}

impl IntervalTree {
    fn get_range(rule: &Rule, field_idx: usize) -> Range<u32> {
        match field_idx {
            0 => Range {
                min: rule.src_ip.min,
                max: rule.src_ip.max,
            },
            1 => Range {
                min: rule.dst_ip.min,
                max: rule.dst_ip.max,
            },
            2 => Range {
                min: rule.src_port.min as u32,
                max: rule.src_port.max as u32,
            },
            3 => Range {
                min: rule.dst_port.min as u32,
                max: rule.dst_port.max as u32,
            },
            4 => Range {
                min: rule.proto.min as u32,
                max: rule.proto.max as u32,
            },
            _ => panic!("Invalid field index"),
        }
    }

    pub fn build(rules: Vec<Rule>, field_idx: usize) -> Self {
        let root = Self::build_recursive(rules, field_idx);
        Self {
            root: Some(Box::new(root)),
            field_idx,
        }
    }

    fn build_recursive(rules: Vec<Rule>, field_idx: usize) -> Node {
        if rules.is_empty() {
            return Node::new(0, Vec::new()); // Dummy empty node? Or handle Option higher up.
        }

        // 1. Find center point (median of all endpoints) to balance the tree
        let mut endpoints = Vec::with_capacity(rules.len() * 2);
        for rule in &rules {
            let range = Self::get_range(rule, field_idx);
            endpoints.push(range.min);
            endpoints.push(range.max);
        }
        endpoints.sort_unstable();
        let center = endpoints[endpoints.len() / 2];

        let mut left_rules = Vec::new();
        let mut right_rules = Vec::new();
        let mut center_rules = Vec::new();

        for rule in rules {
            let range = Self::get_range(&rule, field_idx);
            if range.max < center {
                left_rules.push(rule);
            } else if range.min > center {
                right_rules.push(rule);
            } else {
                // Overlaps center
                center_rules.push(rule);
            }
        }

        let mut node = Node::new(center, center_rules);

        if !left_rules.is_empty() {
            node.left = Some(Box::new(Self::build_recursive(left_rules, field_idx)));
        }
        if !right_rules.is_empty() {
            node.right = Some(Box::new(Self::build_recursive(right_rules, field_idx)));
        }

        node
    }

    pub fn classify_packet<'a>(
        &'a self,
        packet: &crate::packet::FiveTuple,
        val: u32,
    ) -> Option<&'a Rule> {
        self.root
            .as_ref()
            .and_then(|root| Self::query_recursive_packet(root, packet, val))
    }

    fn query_recursive_packet<'a>(
        node: &'a Node,
        packet: &crate::packet::FiveTuple,
        val: u32,
    ) -> Option<&'a Rule> {
        let mut best_match: Option<&Rule> = None;

        // Scan current node's overlap list
        for rule in &node.rules {
            if rule.matches(packet) {
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

        // Check children based on value relative to center
        let child_match = if val < node.center {
            node.left
                .as_ref()
                .and_then(|n| Self::query_recursive_packet(n, packet, val))
        } else if val > node.center {
            node.right
                .as_ref()
                .and_then(|n| Self::query_recursive_packet(n, packet, val))
        } else {
            None
        };

        match (best_match, child_match) {
            (Some(b), Some(c)) => {
                if b.priority < c.priority {
                    Some(b)
                } else {
                    Some(c)
                }
            }
            (Some(b), None) => Some(b),
            (None, Some(c)) => Some(c),
            (None, None) => None,
        }
    }
}
