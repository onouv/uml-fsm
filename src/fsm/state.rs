use crate::fsm::Event;
use std::error::Error;

pub trait State<E: Error> {
    fn on_enter(&self) -> Result<(), E>;
    fn on_exit(&self) -> Result<(), E>;
    fn apply(self, event: &dyn Event) -> Self;
}
