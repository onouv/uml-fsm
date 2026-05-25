use crate::party::Party;
use uml_fsm::fsm::{DispatchEvent, Event};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartyState {
    Gathering,
    Traveling,
    Arrived,
    Disbanded,
}

pub enum PartyEvent {
    AddMember(String),
    RemoveMember(String),
    Depart(String),
    DestinationReached,
    BreakUp,
}

impl Event for PartyEvent {}

#[derive(Debug, Clone)]
pub enum PartyAction {
    StartTravel(String),
    Arrived(String),
    Disbanded,
    TransitionRejected,
}

pub struct PartyMachine {
    state: PartyState,
    party: Party,
}

impl PartyMachine {
    pub fn new(party: Party) -> Self {
        Self {
            state: PartyState::Gathering,
            party,
        }
    }

    pub fn state(&self) -> PartyState {
        self.state
    }

    pub fn context(&self) -> &Party {
        &self.party
    }
}

impl DispatchEvent<PartyEvent, PartyAction> for PartyMachine {
    fn dispatch<Sink>(&mut self, event: PartyEvent, actions: &mut Sink)
    where
        Sink: uml_fsm::fsm::ActionSink<PartyAction>,
    {
        match (self.state, event) {
            (PartyState::Gathering, PartyEvent::AddMember(member)) => {
                self.party.add_member(member);
            }
            (PartyState::Gathering, PartyEvent::RemoveMember(member)) => {
                self.party.remove_member(&member);
            }
            (PartyState::Gathering, PartyEvent::Depart(destination)) => {
                if self.party.can_depart() {
                    self.party.set_destination(destination.clone());
                    self.state = PartyState::Traveling;
                    actions.push(PartyAction::StartTravel(destination));
                } else {
                    actions.push(PartyAction::TransitionRejected);
                }
            }
            (PartyState::Traveling, PartyEvent::DestinationReached) => {
                self.state = PartyState::Arrived;
                let destination = self.party.destination().unwrap_or("unknown").to_string();
                actions.push(PartyAction::Arrived(destination));
            }
            (PartyState::Arrived, PartyEvent::BreakUp) => {
                self.party.disband();
                self.state = PartyState::Disbanded;
                actions.push(PartyAction::Disbanded);
            }
            _ => {
                actions.push(PartyAction::TransitionRejected);
            }
        }
    }
}
