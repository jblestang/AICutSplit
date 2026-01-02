use crate::packet::FiveTuple;

use crate::rule::{Rule, Action};

/// Trait for Packet Classification algorithms
pub trait Classifier {
    /// Build the classifier with a set of rules
    fn build(rules: &[Rule]) -> Self where Self: Sized;
    
    /// Classify a packet (5-tuple) and return the matching Action (if any)
    fn classify(&self, packet: &FiveTuple) -> Option<Action>;
}
