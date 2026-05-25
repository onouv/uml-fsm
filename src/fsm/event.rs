/// A marker trait for events that trigger transitions in the FSM.
pub trait Event {}

pub trait HandleEvent<Ev, Err>
where
    Ev: Event,
{
    type Output;

    fn handle(self, event: Ev) -> Result<Self::Output, Err>;
}
