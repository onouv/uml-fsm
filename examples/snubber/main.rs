use uml_fsm::SnubberNode;

fn main() {
    let node = SnubberNode::new("Node1".to_string(), 100.0);
    println!(
        "Created SnubberNode: {} with projected load: {}",
        node.id(),
        node.projected_load()
    );

    node.run();
}
