#![no_std]
#![deny(warnings)]

extern crate alloc;

pub mod classifier;
pub mod cutsplit;
pub mod hicuts;
pub mod hypersplit;
pub mod linear;
pub mod packet;
pub mod partitionsort;
pub mod rule;
pub mod simulation; // Export simulation
pub mod tss;

// Tests can use std
#[cfg(test)]
extern crate std;
