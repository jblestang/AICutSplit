#![no_std]
#![deny(warnings)]

extern crate alloc;

pub mod packet;
pub mod rule;
pub mod classifier;
pub mod linear;
pub mod cutsplit;
pub mod hicuts;
pub mod hypersplit;
pub mod simulation; // Export simulation
pub mod tss;
pub mod partitionsort;

// Tests can use std
#[cfg(test)]
extern crate std;
