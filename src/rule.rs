use crate::packet::FiveTuple;
use core::fmt;

/// Represents a range of values [min, max] inclusive.
///
/// Used for defining rule matches (e.g. port ranges, IP ranges).
/// A single value is represented as min == max.
/// "Any" (wildcard) is represented as the full range of the type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range<T> {
    /// Minimum value (inclusive)
    pub min: T,
    /// Maximum value (inclusive)
    pub max: T,
}

impl<T: PartialOrd + Copy> Range<T> {
    /// Check if a value is contained within the range.
    pub fn contains(&self, val: T) -> bool {
        val >= self.min && val <= self.max
    }

    /// Create a new range [min, max].
    pub fn new(min: T, max: T) -> Self {
        Self { min, max }
    }
    
    /// Create an exact match range [val, val].
    pub fn exact(val: T) -> Self {
        Self { min: val, max: val }
    }

    /// Create a wildcard range [min, max].
    /// Semantically same as new, but reads better for "Any".
    pub fn any(min: T, max: T) -> Self {
        Self { min, max }
    }
}

/// Rule Action.
///
/// The decision made when a packet matches a rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// Permit the packet to proceed.
    Permit,
    /// Deny/Drop the packet.
    Deny,
}

/// Classification Rule
#[derive(Debug, Clone)]
pub struct Rule {
    pub id: u32,
    pub priority: u32, // Lower value = Higher priority
    pub src_ip: Range<u32>,
    pub dst_ip: Range<u32>,
    pub src_port: Range<u16>,
    pub dst_port: Range<u16>,
    pub proto: Range<u8>,
    pub action: Action,
}

impl Rule {
    /// Check if the rule matches a given 5-tuple
    pub fn matches(&self, tuple: &FiveTuple) -> bool {
        self.src_ip.contains(tuple.src_ip) &&
        self.dst_ip.contains(tuple.dst_ip) &&
        self.src_port.contains(tuple.src_port) &&
        self.dst_port.contains(tuple.dst_port) &&
        self.proto.contains(tuple.proto)
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rule(id={}, pri={}, action={:?})", self.id, self.priority, self.action)
    }
}
