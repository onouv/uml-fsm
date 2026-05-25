/// A marker trait for events that trigger transitions in the FSM.
pub trait Event {}

pub trait HandleEvent<Evt>
where
    Evt: Event,
{
    type Output;

    fn handle(self, event: Evt) -> Self::Output;
}
