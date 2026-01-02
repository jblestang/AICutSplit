# TODO (Completed)

- [x] **Project Initialization**
    - [x] `cargo init --lib --name cutsplit`
    - [x] Configure `Cargo.toml` for `no_std` support.

- [x] **Domain Types**
    - [x] Define `IpAddress` (u32).
    - [x] Define `Range` (min, max).
    - [x] Define `FiveTuple` struct.
    - [x] Define `Rule` struct.
    - [x] Define `Packet` struct and headers (IPV4, TCP, UDP, IGMP).

- [x] **Baseline Implementation**
    - [x] Implement `LinearClassifier`.

- [x] **CutSplit Core Implementation**
    - [x] Define `Node` types (Internal, Leaf).
    - [x] Implement `Builder` with recursive cutting logic.
    - [x] Implement `CutSplitClassifier` traversal.

- [x] **Utils & Simulation**
    - [x] Implement `Simulation` for LAN-WAN rule generation.

- [x] **Benchmarking**
    - [x] Add `criterion` benchmarks.
    - [x] Evaluate gains (up to 63x speedup).

- [x] **Documentation**
    - [x] Add doc comments (~33-50% coverage).
