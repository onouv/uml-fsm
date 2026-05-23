use uml_fsm::SnubberNode;

fn main() {
    let _node = SnubberNode::new("Node1".to_string(), 100.0);
    println!(
        "Created SnubberNode: {} with projected load: {}",
        "Node1", 100.0
    );
}
