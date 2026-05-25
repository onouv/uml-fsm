mod transition;
pub use transition::{Transition, TransitionEffect, TransitionResult};

mod state;
pub use state::State;

mod guard;
pub use guard::Guard;

mod event;
pub use event::Event;
pub use event::HandleEvent;

mod runtime;
pub use runtime::{
    ActionRunner, ActionSink, DispatchEvent, EventSink, EventSource, FixedActionSink,
    FixedEventQueue, NoActions,
};

#[cfg(feature = "heapless")]
pub use runtime::{HeaplessActionSink, HeaplessEventQueue};
