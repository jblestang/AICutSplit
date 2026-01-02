use cutsplit::simulation::Simulation;
use cutsplit::classifier::Classifier;
use cutsplit::linear::LinearClassifier;
use cutsplit::cutsplit::classifier::CutSplitClassifier;

#[test]
fn test_cutsplit_vs_linear_correctness() {
    let mut sim = Simulation::new(12345);
    
    // Test with small rule set
    let rules = sim.generate_rules(100);
    let packets = sim.generate_packets(500);
    
    let linear = LinearClassifier::build(&rules);
    let tree = CutSplitClassifier::build(&rules);
    
    for (i, packet) in packets.iter().enumerate() {
        let res_linear = linear.classify(packet);
        let res_tree = tree.classify(packet);
        
        assert_eq!(res_linear, res_tree, "Mismatch at packet {} {:?}. Linear: {:?}, Tree: {:?}", i, packet, res_linear, res_tree);
    }
}

#[test]
fn test_large_rule_set_correctness() {
    let mut sim = Simulation::new(67890);
    let rules = sim.generate_rules(1000);
    let packets = sim.generate_packets(1000);
    
    let linear = LinearClassifier::build(&rules);
    let tree = CutSplitClassifier::build(&rules);
    
    for (i, packet) in packets.iter().enumerate() {
        let res_linear = linear.classify(packet);
        let res_tree = tree.classify(packet);
        
        assert_eq!(res_linear, res_tree, "Mismatch at packet {} {:?}. Linear: {:?}, Tree: {:?}", i, packet, res_linear, res_tree);
    }
}
