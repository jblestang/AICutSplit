use alloc::vec::Vec;
use alloc::boxed::Box;
use crate::rule::Rule;
use crate::cutsplit::tree::Dimension;

#[derive(Debug, Clone)]
pub enum Node {
    Internal {
        dimension: Dimension,
        pivot: u32,
        left: Box<Node>,
        right: Box<Node>,
    },
    Leaf {
        rules: Vec<Rule>,
    },
}
