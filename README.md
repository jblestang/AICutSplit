# Packet Classification Algorithms in Rust (no_std)

This project implements three high-performance packet classification algorithms: **CutSplit**, **HiCuts**, and **HyperSplit**, along with a baseline **Linear** classifier.

## Algorithms Implemented

| Algorithm | Type | Description |
|-----------|------|-------------|
| **Linear** | Baseline | Sequential search through rules. $O(N)$ complexity. |
| **CutSplit** | Decision Tree | Geometric cuts followed by rule separation. Balanced structure. |
| **HiCuts** | Decision Tree | Multi-way geometric cuts (up to 16 children). Shallower tree, very fast. |
| **HyperSplit**| Decision Tree | Binary space partitioning with rule pushing. Optimized for memory/speed balance. |

## Performance Benchmarks

Benchmarks were run on synthetic LAN-WAN traffic rules.
**HiCuts** proved to be the fastest implementation for this dataset, likely due to its multi-way splitting strategy creating a shallower tree than the binary approaches of CutSplit and HyperSplit.

### Classification Time (Lower is Better)

| Rules | Linear (avg) | CutSplit (avg) | HyperSplit (avg) | **HiCuts (avg)** |
|-------|--------------|----------------|------------------|------------------|
| 100   | ~46 µs       | ~25 µs         | ~22 µs           | **~8 µs**        |
| 350   | ~215 µs      | ~29 µs         | ~28 µs           | **~11 µs**       |
| 850   | ~600 µs      | ~37 µs         | ~34 µs           | **~15 µs**       |
| 1350  | ~985 µs      | ~45 µs         | ~39 µs           | **~18 µs**       |

### Speedup vs Linear (at 1350 rules)

| Algorithm | Speedup |
|-----------|---------|
| **HiCuts** | **~54x** |
| HyperSplit| ~25x    |
| CutSplit  | ~22x    |

## Usage Example

```rust
use cutsplit::rule::{Rule, Range, Action};
use cutsplit::hicuts::classifier::HiCutsClassifier;
use cutsplit::classifier::Classifier;

// Define rules...
let rules = vec![ /* ... */ ];

// Build classifier
let classifier = HiCutsClassifier::build(&rules);

// Classify
let packet = cutsplit::packet::FiveTuple::default(); 
let action = classifier.classify(&packet);
```

## Running Verification

```bash
cargo test   # Verify correctness of all classifiers
cargo bench  # Run performance benchmarks
```
