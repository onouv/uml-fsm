use uml_fsm::fsm::Event;

pub type Member = String;
pub type Destination = String;

pub struct AddMember(pub Member);
impl Event for AddMember {}

pub struct RemoveMember(pub Member);
impl Event for RemoveMember {}

pub struct Depart(pub Destination);
impl Event for Depart {}

pub struct DestinationReached;
impl Event for DestinationReached {}

pub struct BreakUp;
impl Event for BreakUp {}
