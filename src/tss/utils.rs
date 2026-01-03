use alloc::vec::Vec;

/// Represents a Prefix: value/len
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Prefix<T> {
    pub value: T,
    pub len: u32,
}

/// Decompose a range [min, max] into a minimal set of prefixes.
pub fn range_to_prefixes_u32(min: u32, max: u32, bits: u32) -> Vec<Prefix<u32>> {
    let mut prefixes = Vec::new();
    
    // If range is invalid
    if min > max {
        return prefixes;
    }

    let mut current = min;
    while current <= max {
        // Find the longest prefix starting at `current` that is within [current, max]
        
        // Try decreasing length (increasing size)
        // A prefix of length L covers 2^(bits - L) addresses.
        // It's valid if:
        // 1. mask(current, L) == current (start alignment)
        // 2. current + size - 1 <= max (end alignment)
        
        // Optimization: start checking from the max possible size based on alignment
        // The number of trailing zeros defines the max size alignment.
        let trailing_zeros = current.trailing_zeros();
        // The prefix length corresponding to 'trailing_zeros' size would be (bits - trailing_zeros).
        // e.g., if trailing_zeros = 2 (size 4), bits=32, len could be 30.
        // We can't have a size larger than what alignment allows.
        
        let mut best_len = bits;
        
        // Iterating from largest valid block size down to 0 (which is length bits to 0)
        // We actually want smallest length (largest block).
        // Max possible block size based on alignment: 1 << trailing_zeros
        // We also are bounded by 'max'.
        
        // Iterate len from (bits - trailing_zeros) down to 0? No, up to bits.
        // Smallest len = 0 (size 2^32), Largest len = 32 (size 1).
        
        // Start with the alignment constraint
        let alignment_len = if trailing_zeros >= bits { 0 } else { bits - trailing_zeros };
        
        // We also need to fit in [current, max].
        // Let's iterate len from alignment_len to 32.
        // We want the minimal valid len (maximum size).
        
        for l in alignment_len..=bits {
             let size = 1u64 << (bits - l);
             // Check if fits
             if (current as u64) + size - 1 <= (max as u64) {
                 best_len = l;
                 break; 
             }
        }
        
        prefixes.push(Prefix { value: current, len: best_len });
        
        let size = 1u64 << (bits - best_len);
        let next = (current as u64) + size;
        if next > (max as u64) {
            break;
        }
        current = next as u32;
    }
    
    prefixes
}

/// Decompose a u16 range (Ports)
pub fn range_to_prefixes_u16(min: u16, max: u16) -> Vec<Prefix<u16>> {
    let p32 = range_to_prefixes_u32(min as u32, max as u32, 16);
    p32.into_iter().map(|p| Prefix { value: p.value as u16, len: p.len }).collect()
}

/// Decompose a u8 range (Proto)
pub fn range_to_prefixes_u8(min: u8, max: u8) -> Vec<Prefix<u8>> {
    let p32 = range_to_prefixes_u32(min as u32, max as u32, 8);
    p32.into_iter().map(|p| Prefix { value: p.value as u8, len: p.len }).collect()
}
