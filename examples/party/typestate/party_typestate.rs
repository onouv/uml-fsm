use crate::events::{AddMember, BreakUp, Depart, DestinationReached, RemoveMember};
use crate::party::Party;
use uml_fsm::fsm::{
    Event, Guard, HandleEvent, State, Transition, TransitionEffect, TransitionResult,
};

#[derive(Debug, Clone, Copy)]
pub struct Gathering;

#[derive(Debug, Clone, Copy)]
pub struct Traveling;

#[derive(Debug, Clone, Copy)]
pub struct Arrived;

#[derive(Debug, Clone, Copy)]
pub struct Disbanded;

pub struct PartyFsm<S> {
    state: S,
    party: Party,
}

impl<S> PartyFsm<S> {
    pub fn context(&self) -> &Party {
        &self.party
    }

    pub fn into_context(self) -> Party {
        self.party
    }
}

impl PartyFsm<Gathering> {
    pub fn new(party: Party) -> Self {
        let state = Gathering;
        state.on_enter();
        Self { state, party }
    }

    pub fn attendees(&self) -> &[String] {
        self.party.attendees()
    }
}

impl PartyFsm<Traveling> {
    pub fn destination(&self) -> Option<&str> {
        self.party.destination()
    }
}

pub struct AlwaysGuard;

impl<S, Evt> Guard<S, Evt> for AlwaysGuard {
    fn check(&self, _state: &S, _event: &Evt) -> bool {
        true
    }
}

pub struct CanDepartGuard(bool);

impl CanDepartGuard {
    pub fn from_context(party: &Party) -> Self {
        Self(party.can_depart())
    }

    fn can_depart(&self) -> bool {
        self.0
    }
}

impl Guard<Gathering, Depart> for CanDepartGuard {
    fn check(&self, _state: &Gathering, _event: &Depart) -> bool {
        self.can_depart()
    }
}

pub struct LogEffect;

impl<S, Evt> TransitionEffect<S, Evt> for LogEffect
where
    S: State,
    Evt: Event,
{
    fn execute(&self, _state: &S, _event: &Evt) {}
}

impl State for Gathering {
    fn on_enter(&self) {
        println!("enter Gathering");
    }

    fn on_exit(&self) {
        println!("exit Gathering");
    }
}

impl State for Traveling {
    fn on_enter(&self) {
        println!("enter Traveling");
    }

    fn on_exit(&self) {
        println!("exit Traveling");
    }
}

impl State for Arrived {
    fn on_enter(&self) {
        println!("enter Arrived");
    }

    fn on_exit(&self) {
        println!("exit Arrived");
    }
}

impl State for Disbanded {
    fn on_enter(&self) {
        println!("enter Disbanded");
    }
}

impl HandleEvent<AddMember> for PartyFsm<Gathering> {
    type Output = PartyFsm<Gathering>;

    fn handle(self, event: AddMember) -> Self::Output {
        let PartyFsm { state, mut party } = self;
        party.add_member(event.0.clone());

        println!(
            "Gathering handles AddMember({}) -> {:?}",
            event.0,
            party.attendees()
        );

        let transition = Transition::new(Gathering, AlwaysGuard, LogEffect);
        match transition.fire(state, event) {
            TransitionResult::Transitioned(next) => PartyFsm { state: next, party },
            TransitionResult::Stayed(current) => PartyFsm {
                state: current,
                party,
            },
        }
    }
}

impl HandleEvent<RemoveMember> for PartyFsm<Gathering> {
    type Output = PartyFsm<Gathering>;

    fn handle(self, event: RemoveMember) -> Self::Output {
        let PartyFsm { state, mut party } = self;
        party.remove_member(&event.0);

        let transition = Transition::new(Gathering, AlwaysGuard, LogEffect);
        match transition.fire(state, event) {
            TransitionResult::Transitioned(next) => PartyFsm { state: next, party },
            TransitionResult::Stayed(current) => PartyFsm {
                state: current,
                party,
            },
        }
    }
}

impl HandleEvent<Depart> for PartyFsm<Gathering> {
    type Output = DepartureResult;

    fn handle(self, event: Depart) -> Self::Output {
        let PartyFsm { state, mut party } = self;
        let destination = event.0.clone();
        let guard = CanDepartGuard::from_context(&party);

        let transition = Transition::new(Traveling, guard, LogEffect);
        match transition.fire(state, event) {
            TransitionResult::Transitioned(next) => {
                party.set_destination(destination);
                DepartureResult::Traveling(PartyFsm { state: next, party })
            }
            TransitionResult::Stayed(current) => DepartureResult::Gathering(PartyFsm {
                state: current,
                party,
            }),
        }
    }
}

impl HandleEvent<DestinationReached> for PartyFsm<Traveling> {
    type Output = PartyFsm<Arrived>;

    fn handle(self, event: DestinationReached) -> Self::Output {
        let PartyFsm { state, party } = self;

        let transition = Transition::new(Arrived, AlwaysGuard, LogEffect);
        match transition.fire(state, event) {
            TransitionResult::Transitioned(next) => PartyFsm { state: next, party },
            TransitionResult::Stayed(_) => unreachable!("AlwaysGuard cannot stay"),
        }
    }
}

impl HandleEvent<BreakUp> for PartyFsm<Arrived> {
    type Output = PartyFsm<Disbanded>;

    fn handle(self, event: BreakUp) -> Self::Output {
        let PartyFsm { state, mut party } = self;
        party.disband();

        let transition = Transition::new(Disbanded, AlwaysGuard, LogEffect);
        match transition.fire(state, event) {
            TransitionResult::Transitioned(next) => PartyFsm { state: next, party },
            TransitionResult::Stayed(_) => unreachable!("AlwaysGuard cannot stay"),
        }
    }
}

pub enum DepartureResult {
    Gathering(PartyFsm<Gathering>),
    Traveling(PartyFsm<Traveling>),
}
