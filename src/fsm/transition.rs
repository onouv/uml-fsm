use crate::fsm::{Action, Guard, State};

pub struct Transition<R, E>
where
    E: std::error::Error,
{
    from: Box<dyn State<E>>,
    to: Box<dyn State<E>>,
    guard: Box<dyn Guard>,
    action: Box<dyn Action<R, E>>,
}

impl<R, E> Transition<R, E>
where
    E: std::error::Error,
{
    pub fn new(
        from: Box<dyn State<E>>,
        to: Box<dyn State<E>>,
        guard: Box<dyn Guard>,
        action: Box<dyn Action<R, E>>,
    ) -> Self {
        Self {
            from,
            to,
            guard,
            action,
        }
    }

    pub fn on_trigger(self) -> Result<Box<dyn State<E>>, E> {
        if self.guard.check() {
            self.from.on_exit()?;
            self.action.execute()?;
            self.to.on_enter()?;
            Ok(self.to)
        } else {
            Ok(self.from)
        }
    }
}
