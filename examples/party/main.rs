mod party;
use party::{Gathering, GatheringOrTraveling};

mod events;
use events::{AddMember, BreakUp, Depart, DestinationReached};

use uml_fsm::fsm::HandleEvent;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let origin = "Home".to_string();
    let destination = "Beach".to_string();

    let gathering = Gathering::new(origin);
    //let gathering = gathering.handle(AddMember("Alice".to_string()))?;
    //let gathering = gathering.handle(AddMember("Bob".to_string()))?;

    let traveling = match gathering.handle(Depart(destination))? {
        GatheringOrTraveling::Gathering(stayed) => {
            println!("departure blocked, attendees: {}", stayed.attendees().len());
            return Ok(());
        }
        GatheringOrTraveling::Traveling(next) => next,
    };

    println!("destination is {}", traveling.destination());
    let arrived = traveling.handle(DestinationReached)?;
    let _disbanded = arrived.handle(BreakUp)?;

    // Uncommenting the next line is a compile-time error because
    // Disbanded does not implement HandleEvent<AddMember, PartyError>.
    // let _invalid = _disbanded.handle(AddMember("Charlie".to_string()))?;

    Ok(())
}
