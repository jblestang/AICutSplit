use crate::classifier::Classifier;
use crate::packet::FiveTuple;
use crate::rule::{Rule, Action};
use crate::cutsplit::tree::{Node, Dimension};
use crate::cutsplit::builder::Builder;

/// CutSplit Packet Classifier.
///
/// Uses a decision tree (HyperCuts-like) to quickly classify packets.
/// Rules are duplicated into subtrees if they overlap the cut.
pub struct CutSplitClassifier {
    root: Node,
}

impl Classifier for CutSplitClassifier {
    /// Build the classifier.
    ///
    /// Constructs the decision tree using the `Builder` with default settings (threshold=10, depth=20).
    fn build(rules: &[Rule]) -> Self {
        // CutSplit builder params
        // Threshold: typically 8-16 rules for linear scan in leaf
        // Depth: prevent stack overflow
        let builder = Builder::new(10, 20); 
        let root = builder.build(rules);
        Self { root }
    }

    /// Classify the packet using the decision tree.
    fn classify(&self, packet: &FiveTuple) -> Option<Action> {
        let mut current = &self.root;

        loop {
            match current {
                Node::Internal { dimension, cut_val, left, right } => {
                     let val = match dimension {
                        Dimension::SrcIp => packet.src_ip,
                        Dimension::DstIp => packet.dst_ip,
                        Dimension::SrcPort => packet.src_port as u32,
                        Dimension::DstPort => packet.dst_port as u32,
                        Dimension::Proto => packet.proto as u32,
                    };

                    if val < *cut_val {
                        current = left;
                    } else {
                        current = right;
                    }
                },
                Node::Leaf { rules } => {
                    // Linear search in leaf
                    for rule in rules {
                        if rule.matches(packet) {
                            return Some(rule.action);
                        }
                    }
                    return None;
                }
            }
        }
    }
}
