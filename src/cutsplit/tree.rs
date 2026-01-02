use alloc::vec::Vec;
use alloc::boxed::Box;
use crate::rule::Rule;

/// Dimensions to cut on.
///
/// Use to select which field of the 5-tuple to split the search space.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dimension {
    SrcIp,
    DstIp,
    SrcPort,
    DstPort,
    Proto,
}

/// A node in the CutSplit decision tree.
///
/// Can be:
/// - `Internal`: A node that splits traffic based on a dimension and value.
/// - `Leaf`: A node containing a list of rules to match linearly.
#[derive(Debug, Clone)]
pub enum Node {
    /// Internal node performing a cut.
    Internal {
        /// The dimension (field) being compared.
        dimension: Dimension,
        /// The threshold value for the cut.
        /// Left child handles values < cut_val.
        /// Right child handles values >= cut_val.
        cut_val: u32, 
        /// Left child node.
        left: Box<Node>,
        /// Right child node.
        right: Box<Node>,
    },
    /// Leaf node containing final rules.
    Leaf {
        /// Rules that match the path to this leaf.
        /// Should be checked linearly in priority order.
        rules: Vec<Rule>,
    },
}

impl Node {
    /// Returns true if the node is a Leaf.
    pub fn is_leaf(&self) -> bool {
        match self {
            Node::Leaf { .. } => true,
            _ => false,
        }
    }
}
