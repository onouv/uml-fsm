use uml_fsm::fsm::Event;

pub type Member = String;
pub type Destination = String;

pub struct AddMember(Member);
impl Event for AddMember {}
pub struct RemoveMember(Member);
impl Event for RemoveMember {}
pub struct Depart(Destination);
impl Event for Depart {}
pub struct DestinationReached();
impl Event for DestinationReached {}
pub struct BreakUp();
impl Event for BreakUp {}
