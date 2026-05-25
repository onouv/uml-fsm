use crate::fsm::{Action, Event, Guard, State};
use std::marker::PhantomData;

pub enum TransitionResult<T, S> {
    Transitioned(T),
    Stayed(S),
}

pub struct Transition<S, Ev, Next, G, A> {
    target: Next,
    guard: G,
    action: A,
    _marker: PhantomData<(S, Ev)>,
}

impl<S, Ev, Next, G, A> Transition<S, Ev, Next, G, A>
where
    G: Guard<S, Ev>,
{
    pub fn new(target: Next, guard: G, action: A) -> Self {
        Self {
            target,
            guard,
            action,
            _marker: PhantomData,
        }
    }

    pub fn fire<E>(self, state: S, event: Ev) -> Result<TransitionResult<Next, S>, E>
    where
        E: std::error::Error,
        S: State<E>,
        Ev: Event,
        Next: State<E>,
        A: Action<S, Ev, E>,
    {
        if self.guard.check(&state, &event) {
            state.on_exit()?;
            self.action.execute(&state, &event)?;
            let next = self.target;
            next.on_enter()?;
            Ok(TransitionResult::Transitioned(next))
        } else {
            Ok(TransitionResult::Stayed(state))
        }
    }
}
