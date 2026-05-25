#[path = "../party.rs"]
mod party;

mod party_runtime;
use crate::party::Party;
use party_runtime::{PartyAction, PartyEvent, PartyMachine};

use uml_fsm::fsm::{
    ActionRunner, DispatchEvent, EventSink, EventSource, FixedActionSink, FixedEventQueue,
};

struct ConsoleRunner;

impl ActionRunner<PartyAction, PartyEvent> for ConsoleRunner {
    fn run_action(&mut self, action: PartyAction) -> Option<PartyEvent> {
        match action {
            PartyAction::StartTravel(destination) => {
                println!("runner: start traveling to {}", destination);
                Some(PartyEvent::DestinationReached)
            }
            PartyAction::Arrived(destination) => {
                println!("runner: arrived at {}", destination);
                Some(PartyEvent::BreakUp)
            }
            PartyAction::Disbanded => {
                println!("runner: party disbanded");
                None
            }
            PartyAction::TransitionRejected => {
                println!("runner: transition rejected");
                None
            }
        }
    }
}

fn run_event_loop(
    machine: &mut PartyMachine,
    event_queue: &mut FixedEventQueue<'_, PartyEvent>,
    sink: &mut FixedActionSink<'_, PartyAction>,
    runner: &mut ConsoleRunner,
) {
    while let Some(event) = event_queue.pop_event() {
        machine.dispatch(event, sink);

        if sink.overflowed() {
            println!("action sink overflowed");
            return;
        }

        for action in sink.drain() {
            if let Some(next_event) = runner.run_action(action) {
                event_queue.push_event(next_event);
            }
        }

        if event_queue.overflowed() {
            println!("event queue overflowed");
            return;
        }
    }
}

fn main() {
    let mut machine = PartyMachine::new(Party::new("Home".to_string()));
    let mut runner = ConsoleRunner;
    let mut event_slots: [Option<PartyEvent>; 16] = core::array::from_fn(|_| None);
    let mut event_queue = FixedEventQueue::new(&mut event_slots);
    let mut action_slots: [Option<PartyAction>; 8] = core::array::from_fn(|_| None);
    let mut sink = FixedActionSink::new(&mut action_slots);

    println!("party starts at {}", machine.context().origin());

    event_queue.push_event(PartyEvent::AddMember("Alice".to_string()));
    event_queue.push_event(PartyEvent::AddMember("Bob".to_string()));
    event_queue.push_event(PartyEvent::RemoveMember("Nobody".to_string()));
    event_queue.push_event(PartyEvent::Depart("Beach".to_string()));

    run_event_loop(&mut machine, &mut event_queue, &mut sink, &mut runner);

    println!("final state: {:?}", machine.state());
    println!("final attendees: {}", machine.context().attendees().len());
}
