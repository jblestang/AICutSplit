# CutSplit Implementation Walkthrough

I have implemented the CutSplit packet classification algorithm in Rust, optimized for `no_std` environments.

## Features Implemented
- **Core Structures**: `FiveTuple`, `Rule`, `Range` supporting `no_std`.
- **Packet Headers**: Structs for `Ipv4Header`, `TcpHeader`, `UdpHeader`, `IgmpHeader`.
- **Linear Classifier**: Baseline implementation for comparison.
- **CutSplit Classifier**:
    - Decision Tree structure (`Node::Internal`, `Node::Leaf`).
    - Builder with heuristic for picking cut dimensions (SrcIp, DstIp, Ports).
    - Handling of rule duplication/cuts.
- **Simulation**: LAN-WAN traffic simulation for benchmarking.

## Verification & Benchmarks

### Correctness
Unit tests confirm that `CutSplit` produces identical classification results to the `Linear` baseline for random traffic.
Run tests with:
```bash
cargo test
```

### Performance Results
Extensive benchmarking was performed with rule sets ranging from 100 to 10,000 rules. The results demonstrate the scalability of CutSplit.

| Rules | Linear (avg) | CutSplit (avg) | Speedup |
|-------|--------------|----------------|---------|
| 100   | ~46 µs       | ~25 µs         | **1.8x**|
| 600   | ~401 µs      | ~36 µs         | **11x** |
| 1100  | ~785 µs      | ~42 µs         | **18x** |
| 2100  | ~1.76 ms     | ~58 µs         | **30x** |
| 3100  | ~2.85 ms     | ~60 µs         | **47x** |
| 4100  | ~3.78 ms     | ~70 µs         | **54x** |
| 5000  | ~4.79 ms     | ~75 µs         | **63x** |
| 6000  | ~5.80 ms     | ~74 µs         | **78x** |
| 7000  | ~6.78 ms     | ~91 µs         | **74x** |
| 8000  | ~7.89 ms     | ~118 µs        | **66x** |
| 9000  | ~8.97 ms     | ~96 µs         | **93x** |
| 10000 | ~9.78 ms     | ~104 µs        | **94x** |

**Trend Analysis**:
- **Linear Classifier**: Linearly increasing cost (~1ms per 1000 rules).
- **CutSplit**: Logarithmic-like cost. Even at 10,000 rules, classification stays around 100µs.
- **Conclusion**: CutSplit provides massive performance gains (approaching 100x speedup) for large rule sets while maintaining `no_std` compatibility.

Run benchmarks with:
```bash
cargo bench
```

## Usage Example

```rust
use cutsplit::rule::{Rule, Range, Action};
use cutsplit::classifier::Classifier;
use cutsplit::cutsplit::classifier::CutSplitClassifier;

let rules = vec![
    Rule {
        id: 1,
        priority: 1,
        src_ip: Range::exact(0xC0A80001), 
        dst_ip: Range::any(0, u32::MAX),
        src_port: Range::any(0, 65535),
        dst_port: Range::exact(80),
        proto: Range::exact(6), 
        action: Action::Permit,
    }
];

let classifier = CutSplitClassifier::build(&rules);
let packet = cutsplit::packet::FiveTuple::default(); 
let action = classifier.classify(&packet);
```
