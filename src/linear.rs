use crate::classifier::Classifier;
use crate::packet::FiveTuple;
use crate::rule::{Action, Rule};
use alloc::vec::Vec;

pub struct LinearClassifier {
    rules: Vec<Rule>,
}

impl Classifier for LinearClassifier {
    fn build(rules: &[Rule]) -> Self {
        // Sort rules by priority (lower is higher priority)
        let mut sorted_rules = rules.to_vec();
        sorted_rules.sort_by_key(|r| r.priority);

        Self {
            rules: sorted_rules,
        }
    }

    fn classify(&self, packet: &FiveTuple) -> Option<Action> {
        for rule in &self.rules {
            if rule.matches(packet) {
                return Some(rule.action);
            }
        }
        None // Implicit default deny or no match
    }
}
