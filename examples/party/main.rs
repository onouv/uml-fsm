mod party;
use party::Party;

mod events;
use events::PartyEvents;

fn main() {
    let origin = "Home".to_string();
    let destination = "Beach".to_string();

    let mut party = Party::new(origin);
    party = party.apply(PartyEvents::AddAttendee("Alice".to_string()));
    party = party.apply(PartyEvents::AddAttendee("Bob".to_string()));
    party = party.apply(PartyEvents::Depart(destination));
}
