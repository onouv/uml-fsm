pub type Power = f32;
pub struct JoiningNode {
    id: String,
    projected_load: Power,
}
pub type AcceptingNode = JoiningNode;
pub struct Node {
    id: String,
}
pub enum SnubberEvent {
    NodeJoinedSwarm(JoiningNode),
    NodeAcceptanceBy(AcceptingNode),
    NodeLeavingSwarm(Node),
    SwarmButtonPressed,
    LoadButtonPressed,
    LoadMeasured(Power),
    SampleTick,
    JoinSwarmTimeout,
}
