use cutsplit::simulation::Simulation;
use cutsplit::classifier::Classifier;
use cutsplit::linear::LinearClassifier;
use cutsplit::cutsplit::classifier::CutSplitClassifier;
use cutsplit::hicuts::classifier::HiCutsClassifier;
use cutsplit::hypersplit::classifier::HyperSplitClassifier;

#[test]
fn test_all_classifiers_correctness() {
    let mut sim = Simulation::new(12345);
    
    // Test with small rule set
    let rules = sim.generate_rules(100);
    let packets = sim.generate_packets(500);
    
    let linear = LinearClassifier::build(&rules);
    let cutsplit = CutSplitClassifier::build(&rules);
    let hicuts = HiCutsClassifier::build(&rules);
    let hypersplit = HyperSplitClassifier::build(&rules);
    
    for (i, packet) in packets.iter().enumerate() {
        let res_linear = linear.classify(packet);
        let res_cutsplit = cutsplit.classify(packet);
        let res_hicuts = hicuts.classify(packet);
        let res_hypersplit = hypersplit.classify(packet);
        
        assert_eq!(res_linear, res_cutsplit, "CutSplit mismatch at packet {} {:?}. Linear: {:?}, CutSplit: {:?}", i, packet, res_linear, res_cutsplit);
        assert_eq!(res_linear, res_hicuts, "HiCuts mismatch at packet {} {:?}. Linear: {:?}, HiCuts: {:?}", i, packet, res_linear, res_hicuts);
        assert_eq!(res_linear, res_hypersplit, "HyperSplit mismatch at packet {} {:?}. Linear: {:?}, HyperSplit: {:?}", i, packet, res_linear, res_hypersplit);
    }
}

#[test]
fn test_large_rule_set_correctness() {
    let mut sim = Simulation::new(67890);
    let rules = sim.generate_rules(1000);
    let packets = sim.generate_packets(1000);
    
    let linear = LinearClassifier::build(&rules);
    let cutsplit = CutSplitClassifier::build(&rules);
    let hicuts = HiCutsClassifier::build(&rules);
    let hypersplit = HyperSplitClassifier::build(&rules);
    
    for (i, packet) in packets.iter().enumerate() {
        let res_linear = linear.classify(packet);
        let res_cutsplit = cutsplit.classify(packet);
        let res_hicuts = hicuts.classify(packet);
        let res_hypersplit = hypersplit.classify(packet);
        
        assert_eq!(res_linear, res_cutsplit, "CutSplit mismatch at packet {} {:?}. Linear: {:?}, CutSplit: {:?}", i, packet, res_linear, res_cutsplit);
        assert_eq!(res_linear, res_hicuts, "HiCuts mismatch at packet {} {:?}. Linear: {:?}, HiCuts: {:?}", i, packet, res_linear, res_hicuts);
        assert_eq!(res_linear, res_hypersplit, "HyperSplit mismatch at packet {} {:?}. Linear: {:?}, HyperSplit: {:?}", i, packet, res_linear, res_hypersplit);
    }
}
