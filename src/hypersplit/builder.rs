use alloc::vec::Vec;
use alloc::boxed::Box;
use crate::rule::{Rule, Range};
use crate::cutsplit::tree::Dimension;
use crate::hypersplit::tree::Node;

pub struct Builder {
    pub leaf_threshold: usize,
    pub max_depth: usize,
}

impl Builder {
    pub fn new(leaf_threshold: usize, max_depth: usize) -> Self {
        Self { leaf_threshold, max_depth }
    }

    pub fn build(&self, rules: &[Rule]) -> Node {
        self.build_recursive(rules, 0)
    }

    fn build_recursive(&self, rules: &[Rule], depth: usize) -> Node {
        if rules.len() <= self.leaf_threshold || depth >= self.max_depth {
            return Node::Leaf { rules: rules.to_vec() };
        }

        // Find best split
        if let Some((dim, pivot)) = self.find_best_split(rules) {
            let (left_rules, right_rules) = self.split_rules(rules, dim, pivot);
            
            // Optimization: If split doesn't reduce max set size significantly, stop or change strategy.
            // For now, simple recursion.
            if left_rules.len() == rules.len() && right_rules.len() == rules.len() {
                 return Node::Leaf { rules: rules.to_vec() };
            }

            Node::Internal {
                dimension: dim,
                pivot,
                left: Box::new(self.build_recursive(&left_rules, depth + 1)),
                right: Box::new(self.build_recursive(&right_rules, depth + 1)),
            }
        } else {
            Node::Leaf { rules: rules.to_vec() }
        }
    }

    fn find_best_split(&self, rules: &[Rule]) -> Option<(Dimension, u32)> {
        let dimensions = [Dimension::SrcIp, Dimension::DstIp, Dimension::SrcPort, Dimension::DstPort, Dimension::Proto];
        let mut best_score = f32::MAX;
        let mut best_split = None;

        for &dim in &dimensions {
            // Collect candidates
            let mut points = Vec::new();
            for rule in rules {
                let range = self.get_range(rule, dim);
                points.push(range.min);
                points.push(range.max.saturating_add(1));
            }
            points.sort_unstable();
            points.dedup();
            
            // Limit candidates for speed (uniform sampling if too many)
            let step = if points.len() > 16 { points.len() / 16 } else { 1 };
            
            for i in (0..points.len()).step_by(step) {
                let pivot = points[i];
                if pivot == 0 { continue; } // Avoid splitting at 0 if min is 0

                let (l, r) = self.count_split(rules, dim, pivot);
                
                // Avoid empty splits
                if l == 0 || r == 0 { continue; }
                if l == rules.len() && r == rules.len() { continue; }

                // Cost: Max(L, R) roughly approximates worst-case search + penalty for sum (duplication)
                let score = (l.max(r) as f32) + 0.1 * ((l + r) as f32);
                
                if score < best_score {
                    best_score = score;
                    best_split = Some((dim, pivot));
                }
            }
        }
        best_split
    }

    fn split_rules(&self, rules: &[Rule], dim: Dimension, pivot: u32) -> (Vec<Rule>, Vec<Rule>) {
        let mut left = Vec::new();
        let mut right = Vec::new();
        for rule in rules {
            let range = self.get_range(rule, dim);
            if range.min < pivot { left.push(rule.clone()); }
            if range.max >= pivot { right.push(rule.clone()); }
        }
        (left, right)
    }

    fn count_split(&self, rules: &[Rule], dim: Dimension, pivot: u32) -> (usize, usize) {
        let mut l = 0;
        let mut r = 0;
        for rule in rules {
            let range = self.get_range(rule, dim);
            if range.min < pivot { l += 1; }
            if range.max >= pivot { r += 1; }
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
