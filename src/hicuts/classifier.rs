use crate::classifier::Classifier;
use crate::packet::FiveTuple;
use crate::rule::{Rule, Action};
use crate::hicuts::tree::Node;
use crate::hicuts::builder::Builder;
use crate::cutsplit::tree::Dimension;

pub struct HiCutsClassifier {
    root: Node,
}

impl Classifier for HiCutsClassifier {
    fn build(rules: &[Rule]) -> Self {
        let builder = Builder::new(10, 20);
        let root = builder.build(rules);
        Self { root }
    }

    fn classify(&self, packet: &FiveTuple) -> Option<Action> {
        let mut current = &self.root;

        loop {
            match current {
                Node::Internal { dimension, start, step, num_cuts, children } => {
                     let val = match dimension {
                        Dimension::SrcIp => packet.src_ip,
                        Dimension::DstIp => packet.dst_ip,
                        Dimension::SrcPort => packet.src_port as u32,
                        Dimension::DstPort => packet.dst_port as u32,
                        Dimension::Proto => packet.proto as u32,
                    };

                    // Calculate index
                    // idx = (val - start) / step
                    if val < *start {
                        // Should technically not happen if start matches root range 0, but safety check for unexpected ranges?
                        // If outside, usually means defaults or should clamp.
                        return None; 
                    }
                    
                    let offset = val - start;
                    let mut index = offset / step;
                    
                    if index >= *num_cuts {
                        index = num_cuts - 1;
                    }
                    
                    current = &children[index as usize];
                },
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
