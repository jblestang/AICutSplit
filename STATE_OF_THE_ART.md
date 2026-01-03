# State of the Art: Packet Classification Algorithms

This document summarizes the current state of the art in packet classification research, highlighting the algorithms implemented in this library and their theoretical foundations.

## 1. Decision Tree Algorithms

Tree-based algorithms recursively partition the geometric rule space into smaller subspaces.

### HiCuts (Hierarchical Intelligent Cuttings)
*   **Reference**: [Packet Classification using Hierarchical Intelligent Cuttings (2000)](http://yuba.stanford.edu/~nickm/papers/sigcomm2000.pdf)
*   **Mechanism**: Uses local heuristics to cut the space into $N$ equal-sized regions.
*   **Pros**: Fast Classification.
*   **Cons**: Memory explosion for large rule sets; heuristic-dependent.

### HyperSplit
*   **Reference**: [Packet Classification Algorithms: From Theory to Practice (IEEE INFOCOM 2009)](https://ieeexplore.ieee.org/document/5061887)
*   **Mechanism**: Binary decision tree. Uses a "push-up" rule strategy (handling overlapping rules at internal nodes) and memory-optimized node layout.
*   **Pros**: Excellent memory efficiency and speed; scales well to large static rule sets.
*   **Cons**: Slow updates (static).

### CutSplit
*   **Reference**: [CutSplit: A Decision-Tree Combining Cutting and Splitting (IEEE INFOCOM 2018)](https://ieeexplore.ieee.org/document/8464035)
*   **Mechanism**: A hybrid. Uses HiCuts (Cutting) for the upper parts of the tree to separate rules quickly, and HyperSplit (Splitting) for the leaf nodes to handle remaining overlaps efficiently.
*   **Pros**: Balances the depth of the tree with memory usage.

## 2. Decomposition / Tuple Space Algorithms

These algorithms decompose the problem into exact-match lookups.

### Tuple Space Search (TSS)
*   **Reference**: "Packet Classification on Multiple Fields" (SIGCOMM 1999)
*   **Mechanism**: Decomposes rules into Tuples (unique combinations of Field lengths). Each Tuple is searched via a Hash Table.
*   **Pros**: Fast updates ($O(1)$ usually); standard in Open vSwitch.
*   **Cons**: **Tuple Explosion**. Random ranges or IPv6 can create thousands of tuples, making lookup linear in the number of tuples.

### TupleMerge
*   **Reference**: [TupleMerge: Building Online Packet Classifiers by Omitting Bits (IEEE ToN 2019)](https://ieeexplore.ieee.org/document/8038296)
*   **Mechanism**: Relaxes the exact-match requirement. Merges compatible tuples (where one masks fewer bits than another) to reduce the number of hash tables.
*   **Pros**: drastically reduces the number of tables (Lookup $O(T)$) while maintaining fast updates.
*   **Cons**: Slightly more complex to build than TSS.

## 3. Geometric / Hybrid Algorithms

### PartitionSort
*   **Reference**: [A Sorted-Partitioning Approach to Fast and Scalable Dynamic Packet Classification (IEEE ToN 2018)](https://ieeexplore.ieee.org/document/7774710)
*   **Mechanism**: Views classification as a multi-dimensional sorting problem. Partitions rules into "Sortable Rulesets" (Sortable on one dimension). Each partition is indexed by a Multi-dimensional Interval Tree (MIT) or similar structure.
*   **Pros**: Logarithmic classification time ($O(d + \log N)$ per partition). Fast updates. No rule replication.
*   **Cons**: Performance depends on the number of partitions required.

## 4. Machine Learning Approaches (Not Implemented)

### NuevoMatch
*   **Reference**: [A Computational Approach to Packet Classification (SIGCOMM 2020)](https://dl.acm.org/doi/10.1145/3387514.3405886)
*   **Mechanism**: Uses Range-Query Recursive Model Indexes (RQ-RMI) (Neural Networks) to learn the rule distribution and predict match locations.
*   **Pros**: Extremely fast on modern CPUs (AVX).
*   **Cons**: Requires slow offline training; complex to support updates.

### NeuroCuts
*   **Reference**: [NeuroCuts: Packet Classification via Deep Learning-based Tree Construction (SIGCOMM 2019)](https://dl.acm.org/doi/10.1145/3341302.3342091)
*   **Mechanism**: Uses Deep Reinforcement Learning (DRL) to learn the optimal decision tree structure (branching factors, dimensions).
*   **Pros**: Generates highly optimized trees.
*   **Cons**: Very slow training time (hours).
