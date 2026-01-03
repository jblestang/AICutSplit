use crate::cutsplit::tree::Dimension;
use crate::rule::Rule;
use alloc::boxed::Box;
use alloc::vec::Vec;

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
