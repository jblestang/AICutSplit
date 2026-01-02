# Packet Classification Algorithms in Rust (no_std)

This project implements three high-performance packet classification algorithms: **CutSplit**, **HiCuts**, and **HyperSplit**, along with a baseline **Linear** classifier.

## Algorithms Implemented

| Algorithm | Type | Description |
|-----------|------|-------------|
| **Linear** | Baseline | Sequential search through rules. $O(N)$ complexity. |
| **CutSplit** | Decision Tree | Geometric cuts followed by rule separation. Balanced structure. |
| **HiCuts** | Decision Tree | Multi-way geometric cuts. Very fast for small/medium rule sets. |
| **HyperSplit**| Decision Tree | Binary space partitioning. Scales best for large rule sets (>10k). |

## Performance Benchmarks

Benchmarks were run on synthetic LAN-WAN traffic rules up to 20,000 rules.

**Key Findings:**
*   **HiCuts** is fastest for small to medium rule sets (< 10k).
*   **HyperSplit** scales better and becomes the fastest at large rule counts (20k).
*   **CutSplit** provides consistent performance but is generally slower than the other two tree-based methods.
*   **Linear** classifier scales linearly and is orders of magnitude slower (milliseconds vs microseconds).

### Classification Time (Lower is Better)

| Rules | Linear (avg) | CutSplit (avg) | HyperSplit (avg) | HiCuts (avg) |
|-------|--------------|----------------|------------------|--------------|
| 100   | ~46 µs       | ~25 µs         | ~21 µs           | **~8 µs**    |
| 1000  | ~740 µs      | ~40 µs         | ~33 µs           | **~17 µs**   |
| 5000  | ~4.9 ms      | ~85 µs         | **~47 µs**       | **~47 µs**   |
| 10000 | ~10.3 ms     | ~113 µs        | **~59 µs**       | ~63 µs       |
| 20000 | ~21.4 ms     | ~152 µs        | **~66 µs**       | ~94 µs       |

### Speedup vs Linear (at 20,000 rules)

| Algorithm | Speedup |
|-----------|---------|
| **HyperSplit**| **~324x**|
| HiCuts    | ~227x   |
| CutSplit  | ~140x   |

## Usage Example

```rust
use cutsplit::rule::{Rule, Range, Action};
use cutsplit::hypersplit::classifier::HyperSplitClassifier; // Optimized for large sets
use cutsplit::classifier::Classifier;

// Define rules...
let rules = vec![ /* ... */ ];

// Build classifier
let classifier = HyperSplitClassifier::build(&rules);

// Classify
let packet = cutsplit::packet::FiveTuple::default(); 
let action = classifier.classify(&packet);
```

## Running Verification

```bash
cargo test   # Verify correctness of all classifiers
cargo bench  # Run performance benchmarks
```
