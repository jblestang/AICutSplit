//! HyperSplit Classifier Implementation
//!
//! Based on the paper:
//! "Packet Classification Algorithms: From Theory to Practice"
//! Yaxuan Qi, et al. (IEEE INFOCOM 2009)
//! <https://ieeexplore.ieee.org/document/5061887>

use crate::classifier::Classifier;
use crate::cutsplit::tree::Dimension;
use crate::hypersplit::builder::Builder;
use crate::hypersplit::tree::Node;
use crate::packet::FiveTuple;
use crate::rule::{Action, Rule};

pub struct HyperSplitClassifier {
    root: Node,
}

impl Classifier for HyperSplitClassifier {
    fn build(rules: &[Rule]) -> Self {
        // HyperSplit usually builds deeper trees with lower duplicate ratio
        let builder = Builder::new(8, 32);
        let root = builder.build(rules);
        Self { root }
    }

    fn classify(&self, packet: &FiveTuple) -> Option<Action> {
        let mut current = &self.root;

        loop {
            match current {
                Node::Internal {
                    dimension,
                    pivot,
                    left,
                    right,
                } => {
                    let val = match dimension {
                        Dimension::SrcIp => packet.src_ip,
                        Dimension::DstIp => packet.dst_ip,
                        Dimension::SrcPort => packet.src_port as u32,
                        Dimension::DstPort => packet.dst_port as u32,
                        Dimension::Proto => packet.proto as u32,
                    };

                    if val < *pivot {
                        current = left;
                    } else {
                        current = right;
                    }
                }
                Node::Leaf { rules } => {
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
