use alloc::vec::Vec;
use alloc::boxed::Box;
use crate::rule::Rule;
use crate::cutsplit::tree::Dimension; // Reuse Dimension enum

/// A node in the HiCuts decision tree.
#[derive(Debug, Clone)]
pub enum Node {
    Internal {
        /// Dimension to cut on
        dimension: Dimension,
        /// Start of the range covered by this node (for calculating offset)
        start: u32,
        /// End of the range (exclusive or inclusive? usually implied by parent, but helpful for calculation)
        /// Let's store the step size or shift to make classification fast.
        /// If we divide range [min, max] into N cuts, step = (max - min) / N.
        step: u32,
        /// Number of cuts (children len)
        num_cuts: u32,
        /// Children nodes
        children: Vec<Box<Node>>,
    },
    Leaf {
        rules: Vec<Rule>,
    },
}
