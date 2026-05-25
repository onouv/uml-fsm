use crate::events::{AddMember, BreakUp, Depart, DestinationReached, RemoveMember};
use uml_fsm::fsm::{Guard, HandleEvent, State, Transition, TransitionEffect, TransitionResult};

///
/// States represented as structs to be compile time detectable
///

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

pub struct LogEffect;

impl<S, Ev> TransitionEffect<S, Ev> for LogEffect {
    fn execute(&self, _state: &S, _event: &Ev) {}
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

impl State for Gathering {
    fn on_enter(&self) {
        println!("enter Gathering at {}", self.origin);
    }

    fn on_exit(&self) {
        println!("exit Gathering with {} attendees", self.attendees.len());
    }
}

impl State for Traveling {
    fn on_enter(&self) {
        println!(
            "enter Traveling {} -> {} ({} attendees)",
            self.origin,
            self.destination,
            self.attendees.len()
        );
    }

    fn on_exit(&self) {
        println!("exit Traveling to {}", self.destination);
    }
}

impl State for Arrived {
    fn on_enter(&self) {
        println!(
            "enter Arrived at {} ({} attendees)",
            self.destination,
            self.attendees.len()
        );
    }

    fn on_exit(&self) {
        println!("exit Arrived at {}", self.destination);
    }
}

impl State for Disbanded {
    fn on_enter(&self) {
        println!("enter Disbanded");
    }

    fn on_exit(&self) {}
}

impl HandleEvent<AddMember> for Gathering {
    type Output = Gathering;

    fn handle(self, event: AddMember) -> Self::Output {
        let mut attendees = self.attendees.clone();
        attendees.push(event.0.clone());

        let target = Gathering {
            origin: self.origin.clone(),
            attendees,
        };

        let transition = Transition::new(target, AlwaysGuard, LogEffect);
        match transition.fire(self, event) {
            TransitionResult::Transitioned(next) => next,
            TransitionResult::Stayed(current) => current,
        }
    }
}

impl HandleEvent<RemoveMember> for Gathering {
    type Output = Gathering;

    fn handle(self, event: RemoveMember) -> Self::Output {
        let target = Gathering {
            origin: self.origin.clone(),
            attendees: self
                .attendees
                .iter()
                .filter(|m| *m != &event.0)
                .cloned()
                .collect(),
        };

        let transition = Transition::new(target, AlwaysGuard, LogEffect);
        match transition.fire(self, event) {
            TransitionResult::Transitioned(next) => next,
            TransitionResult::Stayed(current) => current,
        }
    }
}

impl HandleEvent<Depart> for Gathering {
    type Output = GatheringOrTraveling;

    fn handle(self, event: Depart) -> Self::Output {
        let target = Traveling {
            origin: self.origin.clone(),
            destination: event.0.clone(),
            attendees: self.attendees.clone(),
        };

        let transition = Transition::new(target, CanDepartGuard, LogEffect);
        match transition.fire(self, event) {
            TransitionResult::Transitioned(next) => GatheringOrTraveling::Traveling(next),
            TransitionResult::Stayed(current) => GatheringOrTraveling::Gathering(current),
        }
    }
}

impl HandleEvent<DestinationReached> for Traveling {
    type Output = Arrived;

    fn handle(self, event: DestinationReached) -> Self::Output {
        let target = Arrived {
            destination: self.destination.clone(),
            attendees: self.attendees.clone(),
        };

        let transition = Transition::new(target, AlwaysGuard, LogEffect);
        match transition.fire(self, event) {
            TransitionResult::Transitioned(next) => next,
            TransitionResult::Stayed(_) => unreachable!("AlwaysGuard cannot stay"),
        }
    }
}

impl HandleEvent<BreakUp> for Arrived {
    type Output = Disbanded;

    fn handle(self, event: BreakUp) -> Self::Output {
        let transition = Transition::new(Disbanded, AlwaysGuard, LogEffect);
        match transition.fire(self, event) {
            TransitionResult::Transitioned(next) => next,
            TransitionResult::Stayed(_) => unreachable!("AlwaysGuard cannot stay"),
        }
    }
}

pub enum GatheringOrTraveling {
    Gathering(Gathering),
    Traveling(Traveling),
}
