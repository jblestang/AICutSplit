use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use cutsplit::simulation::Simulation;
use cutsplit::classifier::Classifier;
use cutsplit::linear::LinearClassifier;
use cutsplit::cutsplit::classifier::CutSplitClassifier;
use cutsplit::hicuts::classifier::HiCutsClassifier;
use cutsplit::hypersplit::classifier::HyperSplitClassifier;
use cutsplit::tss::classifier::TSSClassifier;
use cutsplit::partitionsort::classifier::PartitionSortClassifier;
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
        let cutsplit = CutSplitClassifier::build(&rules);
        let hicuts = HiCutsClassifier::build(&rules);
        let hypersplit = HyperSplitClassifier::build(&rules);
        let tss = TSSClassifier::build(&rules);
        let ps = PartitionSortClassifier::build(&rules);
        
        group.bench_function(format!("Linear/{}", n_rules), |b| {
            b.iter(|| {
                for p in &packets {
                    linear.classify(p);
                }
            })
        });
        
        group.bench_function(format!("CutSplit/{}", n_rules), |b| {
            b.iter(|| {
                for p in &packets {
                    cutsplit.classify(p);
                }
            })
        });

        group.bench_function(format!("HiCuts/{}", n_rules), |b| {
            b.iter(|| {
                for p in &packets {
                    hicuts.classify(p);
                }
            })
        });

        group.bench_function(format!("HyperSplit/{}", n_rules), |b| {
            b.iter(|| {
                for p in &packets {
                    hypersplit.classify(p);
                }
            })
        });

        group.bench_function(format!("TSS/{}", n_rules), |b| {
            b.iter(|| {
                for p in &packets {
                    tss.classify(p);
                }
            })
        });

        group.bench_function(format!("PartitionSort/{}", n_rules), |b| {
            b.iter(|| {
                for p in &packets {
                    ps.classify(p);
                }
            })
        });
    }
    group.finish();
}

criterion_group!(benches, benchmark_classification);
criterion_main!(benches);
