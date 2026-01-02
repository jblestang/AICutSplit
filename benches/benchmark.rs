use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use cutsplit::simulation::Simulation;
use cutsplit::classifier::Classifier;
use cutsplit::linear::LinearClassifier;
use cutsplit::cutsplit::classifier::CutSplitClassifier; // Path might need partial adjustment via lib.rs export?
// cutsplit::cutsplit::classifier::CutSplitClassifier is ... lib->cutsplit->classifier->CSClassifier.
// But lib.rs has `pub mod cutsplit`. And `cutsplit/mod.rs` has `pub mod classifier`.
// So usage is `cutsplit::cutsplit::classifier::CutSplitClassifier`.

fn benchmark_classification(c: &mut Criterion) {
    let mut sim = Simulation::new(42); // Deterministic seed
    
    // Generate rule counts based on user request:
    // - Step of 100 under 100 rules (Start at 100)
    // - Step of 250 after 100 upto 5000
    // - Step of 1000 up to 10000
    let mut rule_counts = Vec::new();
    
    // "step of 100 under 100 rules" -> 100
    rule_counts.push(100);

    // "step of 250 after 100 upto 5000 rules"
    let mut current = 100 + 250;
    while current <= 5000 {
        rule_counts.push(current);
        current += 250;
    }
    // Ensure 5000 is hitting a boundary if we want a clean cut, 
    // but 100 + 19*250 = 4850. Next is 5100.
    // User said "upto 5000". So < 5000 logic is correct. 
    // We can explicitly add 5000 if we consider it a major milestone,
    // but sticking to the "step" logic strictly: 4850 is the last one.
    // Let's add 5000 as a bridge to the next step if strictly needed?
    // "step of 1000 up to 10000" usually starts from 5000.
    // Let's reset current to 5000 base for the next phase or continue?
    // "then by step of 1000".
    // I will start the next phase at 5000 + 1000 = 6000.
    // But maybe I should include 5000? 
    // Let's include 5000 explicitly as it's the "upto" boundary.
    if *rule_counts.last().unwrap() != 5000 {
        rule_counts.push(5000);
    }

    // "upto 10000" with step 1000
    let mut current = 6000;
    while current <= 10000 {
        rule_counts.push(current);
        current += 1000;
    }
    
    let mut group = c.benchmark_group("Classification");
    // Set a lower sample size/time to accommodate many steps if needed
    group.sample_size(50); 
    
    for &n_rules in &rule_counts {
        let rules = sim.generate_rules(n_rules);
        let packets = sim.generate_packets(1000); // 1000 packets for throughput test
        
        // Build Classifiers
        let linear = LinearClassifier::build(&rules);
        let tree = CutSplitClassifier::build(&rules);
        
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
    }
    group.finish();
}

criterion_group!(benches, benchmark_classification);
criterion_main!(benches);
