use crate::fsm::{Action, Guard, State};

pub enum TransitionResult<T, S> {
    Transitioned(T),
    Stayed(S),
}

pub struct Transition<F, T, G, A> {
    from: F,
    to: T,
    guard: G,
    action: A,
}

impl<F, T, G, A> Transition<F, T, G, A>
where
    G: Guard,
{
    pub fn new(from: F, to: T, guard: G, action: A) -> Self {
        Self {
            from,
            to,
            guard,
            action,
        }
    }

    pub fn on_trigger<R, E>(self) -> Result<TransitionResult<T, F>, E>
    where
        E: std::error::Error,
        F: State<E>,
        T: State<E>,
        A: Action<R, E>,
    {
        if self.guard.check() {
            self.from.on_exit()?;
            self.action.execute()?;
            self.to.on_enter()?;
            Ok(TransitionResult::Transitioned(self.to))
        } else {
            Ok(TransitionResult::Stayed(self.from))
        }
    }
}
