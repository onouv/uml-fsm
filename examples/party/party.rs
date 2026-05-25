use crate::events::{AddMember, BreakUp, Depart, DestinationReached, RemoveMember};
use thiserror::Error;
use uml_fsm::fsm::{Action, Guard, HandleEvent, State, Transition, TransitionResult};

#[derive(Debug, Clone)]
pub struct Gathering {
    origin: String,
    attendees: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Traveling {
    origin: String,
    destination: String,
    attendees: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Arrived {
    destination: String,
    attendees: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Disbanded;

#[derive(Debug, Error)]
pub enum PartyError {
    #[error("cannot leave without attendees")]
    NoAttendees,
}

pub struct AlwaysGuard;

impl<S, Ev> Guard<S, Ev> for AlwaysGuard {
    fn check(&self, _state: &S, _event: &Ev) -> bool {
        true
    }
}

pub struct CanDepartGuard;

impl Guard<Gathering, Depart> for CanDepartGuard {
    fn check(&self, state: &Gathering, _event: &Depart) -> bool {
        !state.attendees.is_empty()
    }
}

pub struct LogAction;

impl<S, Ev> Action<S, Ev, PartyError> for LogAction {
    fn execute(&self, _state: &S, _event: &Ev) -> Result<(), PartyError> {
        Ok(())
    }
}

impl Gathering {
    pub fn new(origin: String) -> Self {
        Self {
            origin,
            attendees: Vec::new(),
        }
    }

    pub fn attendees(&self) -> &[String] {
        &self.attendees
    }
}

impl Traveling {
    pub fn destination(&self) -> &str {
        &self.destination
    }
}

impl Arrived {
    pub fn attendees(&self) -> &[String] {
        &self.attendees
    }
}

impl State<PartyError> for Gathering {
    fn on_enter(&self) -> Result<(), PartyError> {
        println!("enter Gathering at {}", self.origin);
        Ok(())
    }

    fn on_exit(&self) -> Result<(), PartyError> {
        println!("exit Gathering with {} attendees", self.attendees.len());
        Ok(())
    }
}

impl State<PartyError> for Traveling {
    fn on_enter(&self) -> Result<(), PartyError> {
        println!(
            "enter Traveling {} -> {} ({} attendees)",
            self.origin,
            self.destination,
            self.attendees.len()
        );
        Ok(())
    }

    fn on_exit(&self) -> Result<(), PartyError> {
        println!("exit Traveling to {}", self.destination);
        Ok(())
    }
}

impl State<PartyError> for Arrived {
    fn on_enter(&self) -> Result<(), PartyError> {
        println!(
            "enter Arrived at {} ({} attendees)",
            self.destination,
            self.attendees.len()
        );
        Ok(())
    }

    fn on_exit(&self) -> Result<(), PartyError> {
        println!("exit Arrived at {}", self.destination);
        Ok(())
    }
}

impl State<PartyError> for Disbanded {
    fn on_enter(&self) -> Result<(), PartyError> {
        println!("enter Disbanded");
        Ok(())
    }

    fn on_exit(&self) -> Result<(), PartyError> {
        Ok(())
    }
}

impl HandleEvent<AddMember, PartyError> for Gathering {
    type Output = Gathering;

    fn handle(self, event: AddMember) -> Result<Self::Output, PartyError> {
        let mut attendees = self.attendees.clone();
        attendees.push(event.0.clone());

        let target = Gathering {
            origin: self.origin.clone(),
            attendees,
        };

        let transition = Transition::new(target, AlwaysGuard, LogAction);
        match transition.fire(self, event)? {
            TransitionResult::Transitioned(next) => Ok(next),
            TransitionResult::Stayed(current) => Ok(current),
        }
    }
}

impl HandleEvent<RemoveMember, PartyError> for Gathering {
    type Output = Gathering;

    fn handle(self, event: RemoveMember) -> Result<Self::Output, PartyError> {
        let target = Gathering {
            origin: self.origin.clone(),
            attendees: self
                .attendees
                .iter()
                .filter(|m| *m != &event.0)
                .cloned()
                .collect(),
        };

        let transition = Transition::new(target, AlwaysGuard, LogAction);
        match transition.fire(self, event)? {
            TransitionResult::Transitioned(next) => Ok(next),
            TransitionResult::Stayed(current) => Ok(current),
        }
    }
}

impl HandleEvent<Depart, PartyError> for Gathering {
    type Output = GatheringOrTraveling;

    fn handle(self, event: Depart) -> Result<Self::Output, PartyError> {
        let target = Traveling {
            origin: self.origin.clone(),
            destination: event.0.clone(),
            attendees: self.attendees.clone(),
        };

        let transition = Transition::new(target, CanDepartGuard, LogAction);
        match transition.fire(self, event)? {
            TransitionResult::Transitioned(next) => Ok(GatheringOrTraveling::Traveling(next)),
            TransitionResult::Stayed(current) => Ok(GatheringOrTraveling::Gathering(current)),
        }
    }
}

impl HandleEvent<DestinationReached, PartyError> for Traveling {
    type Output = Arrived;

    fn handle(self, event: DestinationReached) -> Result<Self::Output, PartyError> {
        let target = Arrived {
            destination: self.destination.clone(),
            attendees: self.attendees.clone(),
        };

        let transition = Transition::new(target, AlwaysGuard, LogAction);
        match transition.fire(self, event)? {
            TransitionResult::Transitioned(next) => Ok(next),
            TransitionResult::Stayed(_) => unreachable!("AlwaysGuard cannot stay"),
        }
    }
}

impl HandleEvent<BreakUp, PartyError> for Arrived {
    type Output = Disbanded;

    fn handle(self, event: BreakUp) -> Result<Self::Output, PartyError> {
        let transition = Transition::new(Disbanded, AlwaysGuard, LogAction);
        match transition.fire(self, event)? {
            TransitionResult::Transitioned(next) => Ok(next),
            TransitionResult::Stayed(_) => unreachable!("AlwaysGuard cannot stay"),
        }
    }
}

pub enum GatheringOrTraveling {
    Gathering(Gathering),
    Traveling(Traveling),
}
