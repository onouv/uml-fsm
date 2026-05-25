use crate::fsm::{Event, Guard, State};
use std::marker::PhantomData;
pub trait TransitionEffect<S, Evt, Err>
where
    Err: std::error::Error,
{
    fn execute(&self, state: &S, event: &Evt) -> Result<(), Err>;
}
pub enum TransitionResult<T, S> {
    Transitioned(T),
    Stayed(S),
}

pub struct Transition<S, Evt, Next, G, Eff> {
    target: Next,
    guard: G,
    effect: Eff,
    _marker: PhantomData<(S, Evt)>,
}

impl<S, Evt, Next, G, Eff> Transition<S, Evt, Next, G, Eff>
where
    G: Guard<S, Evt>,
{
    pub fn new(target: Next, guard: G, effect: Eff) -> Self {
        Self {
            target,
            guard,
            effect,
            _marker: PhantomData,
        }
    }

    pub fn fire<Err>(self, state: S, event: Evt) -> Result<TransitionResult<Next, S>, Err>
    where
        Err: std::error::Error,
        S: State<Err>,
        Evt: Event,
        Next: State<Err>,
        Eff: TransitionEffect<S, Evt, Err>,
    {
        if self.guard.check(&state, &event) {
            state.on_exit()?;
            self.effect.execute(&state, &event)?;
            let next = self.target;
            next.on_enter()?;
            Ok(TransitionResult::Transitioned(next))
        } else {
            Ok(TransitionResult::Stayed(state))
        }
    }
}
