use crate::fsm::{Event, Guard, State};
use core::marker::PhantomData;
pub trait TransitionEffect<S, Evt>
where
    S: State,
    Evt: Event,
{
    fn execute(&self, state: &S, event: &Evt);
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

    pub fn fire(self, state: S, event: Evt) -> TransitionResult<Next, S>
    where
        S: State,
        Evt: Event,
        Next: State,
        Eff: TransitionEffect<S, Evt>,
    {
        if self.guard.check(&state, &event) {
            state.on_exit();
            self.effect.execute(&state, &event);
            let next = self.target;
            next.on_enter();
            TransitionResult::Transitioned(next)
        } else {
            TransitionResult::Stayed(state)
        }
    }
}
