/// A marker trait for events that trigger transitions in the FSM.
pub trait Event {}

pub trait EventHandler<E: Event> {
    fn handle(&self, event: &E);
}
