#[path = "../events.rs"]
mod events;
#[path = "../party.rs"]
mod party;
mod party_typestate;

use crate::events::{AddMember, BreakUp, Depart, DestinationReached};
use crate::party::Party;
use crate::party_typestate::{DepartureResult, PartyFsm};

use uml_fsm::fsm::HandleEvent;

fn main() {
    let origin = "Home".to_string();
    let destination = "Beach".to_string();

    let party = Party::new(origin);
    let gathering = PartyFsm::new(party);
    println!("party starts at {}", gathering.context().origin());
    let gathering = gathering.handle(AddMember("Alice".to_string()));
    let gathering = gathering.handle(AddMember("Bob".to_string()));

    let traveling = match gathering.handle(Depart(destination)) {
        DepartureResult::Gathering(stayed) => {
            println!("departure blocked, attendees: {:?}", stayed.attendees());
            return;
        }
        DepartureResult::Traveling(next) => next,
    };

    println!("destination is {:?}", traveling.destination());
    let arrived = traveling.handle(DestinationReached);
    let disbanded = arrived.handle(BreakUp);
    let final_party = disbanded.into_context();
    println!("party disbanded, attendees: {:?}", final_party.attendees());

    // Uncommenting the next line is a compile-time error because
    // Disbanded does not implement HandleEvent<AddMember>.
    // let _invalid = _disbanded.handle(AddMember("Charlie".to_string()));
}
