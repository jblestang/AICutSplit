use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use cutsplit::simulation::Simulation;
use cutsplit::classifier::Classifier;
use cutsplit::linear::LinearClassifier;
use cutsplit::cutsplit::classifier::CutSplitClassifier;
use cutsplit::hicuts::classifier::HiCutsClassifier;
use cutsplit::hypersplit::classifier::HyperSplitClassifier; // Path might need partial adjustment via lib.rs export?
// cutsplit::cutsplit::classifier::CutSplitClassifier is ... lib->cutsplit->classifier->CSClassifier.
// But lib.rs has `pub mod cutsplit`. And `cutsplit/mod.rs` has `pub mod classifier`.
// So usage is `cutsplit::cutsplit::classifier::CutSplitClassifier`.

fn benchmark_classification(c: &mut Criterion) {
    let mut sim = Simulation::new(42); // Deterministic seed
    
    // Benchmark steps requested by user
    let rule_counts = vec![
        100, 300, 500, 700, 900, 1000, 
        3000, 5000, 7000, 9000, 10000, 
        20000
    ];
    
    let mut group = c.benchmark_group("Classification");
    // Set a lower sample size/time to accommodate many steps if needed
    group.sample_size(50); 
    
    for &n_rules in &rule_counts {
        let rules = sim.generate_rules(n_rules);
        let packets = sim.generate_packets(1000); // 1000 packets for throughput test
        
        // Build Classifiers
        let linear = LinearClassifier::build(&rules);
        let tree = CutSplitClassifier::build(&rules);
        let hicuts = HiCutsClassifier::build(&rules);
        let hypersplit = HyperSplitClassifier::build(&rules);
        
        group.bench_with_input(BenchmarkId::new("Linear", n_rules), &packets, |b, pkts| {
            b.iter(|| {
                for pkt in pkts {
                    linear.classify(pkt);
                }
            })
        });
        
        group.bench_with_input(BenchmarkId::new("CutSplit", n_rules), &packets, |b, pkts| {
            b.iter(|| {
                for pkt in pkts {
                    tree.classify(pkt);
                }
            })
        });

        group.bench_with_input(BenchmarkId::new("HiCuts", n_rules), &packets, |b, pkts| {
            b.iter(|| {
                for pkt in pkts {
                    hicuts.classify(pkt);
                }
            })
        });

        group.bench_with_input(BenchmarkId::new("HyperSplit", n_rules), &packets, |b, pkts| {
            b.iter(|| {
                for pkt in pkts {
                    hypersplit.classify(pkt);
                }
            })
        });
    }
    group.finish();
}

criterion_group!(benches, benchmark_classification);
criterion_main!(benches);
