use crate::fsm::Event;

/// Collects actions emitted by the FSM core.
///
/// Embedded profiles can use a fixed-capacity sink while server profiles can use
/// dynamic collections.
pub trait ActionSink<Act> {
    fn push(&mut self, action: Act);
}

/// Accepts events from producers (for example user input or action runners).
pub trait EventSink<Evt>
where
    Evt: Event,
{
    fn push_event(&mut self, event: Evt);
}

/// Supplies events to the single dispatcher loop.
pub trait EventSource<Evt>
where
    Evt: Event,
{
    fn pop_event(&mut self) -> Option<Evt>;
}

/// A sink that intentionally drops all emitted actions.
pub struct NoActions;

impl<Act> ActionSink<Act> for NoActions {
    fn push(&mut self, _action: Act) {}
}

/// A fixed-capacity FIFO queue for domain events.
///
/// Backed by caller-provided storage and suitable for `no_std` use.
pub struct FixedEventQueue<'a, Evt> {
    slots: &'a mut [Option<Evt>],
    head: usize,
    len: usize,
    overflowed: bool,
}

impl<'a, Evt> FixedEventQueue<'a, Evt>
where
    Evt: Event,
{
    pub fn new(slots: &'a mut [Option<Evt>]) -> Self {
        Self {
            slots,
            head: 0,
            len: 0,
            overflowed: false,
        }
    }

    pub fn overflowed(&self) -> bool {
        self.overflowed
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<Evt> EventSink<Evt> for FixedEventQueue<'_, Evt>
where
    Evt: Event,
{
    fn push_event(&mut self, event: Evt) {
        let cap = self.slots.len();
        if self.len >= cap {
            self.overflowed = true;
            return;
        }

        if cap == 0 {
            self.overflowed = true;
            return;
        }

        let tail = (self.head + self.len) % cap;
        self.slots[tail] = Some(event);
        self.len += 1;
    }
}

impl<Evt> EventSource<Evt> for FixedEventQueue<'_, Evt>
where
    Evt: Event,
{
    fn pop_event(&mut self) -> Option<Evt> {
        if self.len == 0 {
            return None;
        }

        let cap = self.slots.len();
        if cap == 0 {
            self.len = 0;
            return None;
        }

        let event = self.slots[self.head].take();
        self.head = (self.head + 1) % cap;
        self.len -= 1;
        event
    }
}

/// A fixed-capacity sink backed by a caller-provided slice of `Option<Act>`.
///
/// This avoids allocation and works in `no_std` environments.
pub struct FixedActionSink<'a, Act> {
    slots: &'a mut [Option<Act>],
    len: usize,
    overflowed: bool,
}

impl<'a, Act> FixedActionSink<'a, Act> {
    pub fn new(slots: &'a mut [Option<Act>]) -> Self {
        Self {
            slots,
            len: 0,
            overflowed: false,
        }
    }

    pub fn overflowed(&self) -> bool {
        self.overflowed
    }

    pub fn drain(&mut self) -> impl Iterator<Item = Act> + '_ {
        let len = self.len;
        self.len = 0;
        self.slots[..len].iter_mut().filter_map(|slot| slot.take())
    }
}

impl<Act> ActionSink<Act> for FixedActionSink<'_, Act> {
    fn push(&mut self, action: Act) {
        if self.len < self.slots.len() {
            self.slots[self.len] = Some(action);
            self.len += 1;
        } else {
            self.overflowed = true;
        }
    }
}

/// A `heapless`-backed FIFO queue for domain events.
#[cfg(feature = "heapless")]
pub struct HeaplessEventQueue<Evt, const N: usize>
where
    Evt: Event,
{
    queue: heapless::Deque<Evt, N>,
    overflowed: bool,
}

#[cfg(feature = "heapless")]
impl<Evt, const N: usize> HeaplessEventQueue<Evt, N>
where
    Evt: Event,
{
    pub fn new() -> Self {
        Self {
            queue: heapless::Deque::new(),
            overflowed: false,
        }
    }

    pub fn overflowed(&self) -> bool {
        self.overflowed
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

#[cfg(feature = "heapless")]
impl<Evt, const N: usize> EventSink<Evt> for HeaplessEventQueue<Evt, N>
where
    Evt: Event,
{
    fn push_event(&mut self, event: Evt) {
        if self.queue.push_back(event).is_err() {
            self.overflowed = true;
        }
    }
}

#[cfg(feature = "heapless")]
impl<Evt, const N: usize> EventSource<Evt> for HeaplessEventQueue<Evt, N>
where
    Evt: Event,
{
    fn pop_event(&mut self) -> Option<Evt> {
        self.queue.pop_front()
    }
}

/// A `heapless`-backed action sink with FIFO draining.
#[cfg(feature = "heapless")]
pub struct HeaplessActionSink<Act, const N: usize> {
    queue: heapless::Deque<Act, N>,
    overflowed: bool,
}

#[cfg(feature = "heapless")]
impl<Act, const N: usize> HeaplessActionSink<Act, N> {
    pub fn new() -> Self {
        Self {
            queue: heapless::Deque::new(),
            overflowed: false,
        }
    }

    pub fn overflowed(&self) -> bool {
        self.overflowed
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn drain(&mut self) -> impl Iterator<Item = Act> + '_ {
        core::iter::from_fn(move || self.queue.pop_front())
    }
}

#[cfg(feature = "heapless")]
impl<Act, const N: usize> ActionSink<Act> for HeaplessActionSink<Act, N> {
    fn push(&mut self, action: Act) {
        if self.queue.push_back(action).is_err() {
            self.overflowed = true;
        }
    }
}

/// Core FSM boundary (Option 1): consume a typed event and emit typed actions.
pub trait DispatchEvent<Evt, Act>
where
    Evt: Event,
{
    fn dispatch<Sink>(&mut self, event: Evt, actions: &mut Sink)
    where
        Sink: ActionSink<Act>;
}

/// Runtime boundary: execute one action and optionally return a follow-up event.
pub trait ActionRunner<Act, Evt>
where
    Evt: Event,
{
    fn run_action(&mut self, action: Act) -> Option<Evt>;
}

#[cfg(test)]
mod tests {
    use super::{ActionSink, EventSink, EventSource, FixedActionSink, FixedEventQueue};
    use crate::fsm::Event;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct TestEvent(u8);

    impl Event for TestEvent {}

    #[test]
    fn fixed_event_queue_is_fifo() {
        let mut slots = [None, None, None, None];
        let mut queue = FixedEventQueue::new(&mut slots);

        queue.push_event(TestEvent(1));
        queue.push_event(TestEvent(2));
        queue.push_event(TestEvent(3));

        assert_eq!(queue.pop_event(), Some(TestEvent(1)));
        assert_eq!(queue.pop_event(), Some(TestEvent(2)));
        assert_eq!(queue.pop_event(), Some(TestEvent(3)));
        assert_eq!(queue.pop_event(), None);
    }

    #[test]
    fn fixed_event_queue_overflow_is_flagged_and_safe() {
        let mut slots = [None, None];
        let mut queue = FixedEventQueue::new(&mut slots);

        queue.push_event(TestEvent(1));
        queue.push_event(TestEvent(2));
        queue.push_event(TestEvent(3));

        assert!(queue.overflowed());
        assert_eq!(queue.len(), 2);
        assert_eq!(queue.pop_event(), Some(TestEvent(1)));
        assert_eq!(queue.pop_event(), Some(TestEvent(2)));
        assert_eq!(queue.pop_event(), None);
    }

    #[test]
    fn fixed_event_queue_zero_capacity_never_panics() {
        let mut slots: [Option<TestEvent>; 0] = [];
        let mut queue = FixedEventQueue::new(&mut slots);

        queue.push_event(TestEvent(1));

        assert!(queue.overflowed());
        assert!(queue.is_empty());
        assert_eq!(queue.pop_event(), None);
    }

    #[test]
    fn fixed_action_sink_drain_preserves_insert_order() {
        let mut slots = [None, None, None];
        let mut sink = FixedActionSink::new(&mut slots);

        sink.push(10_u8);
        sink.push(20_u8);

        let mut drained = sink.drain();
        assert_eq!(drained.next(), Some(10));
        assert_eq!(drained.next(), Some(20));
        assert_eq!(drained.next(), None);
        drop(drained);

        let mut drained_again = sink.drain();
        assert_eq!(drained_again.next(), None);
    }

    #[test]
    fn fixed_action_sink_overflow_is_flagged_and_safe() {
        let mut slots = [None, None];
        let mut sink = FixedActionSink::new(&mut slots);

        sink.push(1_u8);
        sink.push(2_u8);
        sink.push(3_u8);

        assert!(sink.overflowed());
        let mut drained = sink.drain();
        assert_eq!(drained.next(), Some(1));
        assert_eq!(drained.next(), Some(2));
        assert_eq!(drained.next(), None);
    }
}

#[cfg(all(test, feature = "heapless"))]
mod heapless_tests {
    use super::{ActionSink, EventSink, EventSource, HeaplessActionSink, HeaplessEventQueue};
    use crate::fsm::Event;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct TestEvent(u8);

    impl Event for TestEvent {}

    #[test]
    fn heapless_event_queue_is_fifo() {
        let mut queue = HeaplessEventQueue::<TestEvent, 4>::new();

        queue.push_event(TestEvent(1));
        queue.push_event(TestEvent(2));
        queue.push_event(TestEvent(3));

        assert_eq!(queue.pop_event(), Some(TestEvent(1)));
        assert_eq!(queue.pop_event(), Some(TestEvent(2)));
        assert_eq!(queue.pop_event(), Some(TestEvent(3)));
        assert_eq!(queue.pop_event(), None);
    }

    #[test]
    fn heapless_event_queue_overflow_is_flagged() {
        let mut queue = HeaplessEventQueue::<TestEvent, 2>::new();

        queue.push_event(TestEvent(1));
        queue.push_event(TestEvent(2));
        queue.push_event(TestEvent(3));

        assert!(queue.overflowed());
        assert_eq!(queue.len(), 2);
        assert_eq!(queue.pop_event(), Some(TestEvent(1)));
        assert_eq!(queue.pop_event(), Some(TestEvent(2)));
        assert_eq!(queue.pop_event(), None);
    }

    #[test]
    fn heapless_action_sink_drain_preserves_order() {
        let mut sink = HeaplessActionSink::<u8, 3>::new();

        sink.push(10);
        sink.push(20);

        let mut drained = sink.drain();
        assert_eq!(drained.next(), Some(10));
        assert_eq!(drained.next(), Some(20));
        assert_eq!(drained.next(), None);
    }

    #[test]
    fn heapless_action_sink_overflow_is_flagged() {
        let mut sink = HeaplessActionSink::<u8, 2>::new();

        sink.push(1);
        sink.push(2);
        sink.push(3);

        assert!(sink.overflowed());
        assert_eq!(sink.len(), 2);
        let mut drained = sink.drain();
        assert_eq!(drained.next(), Some(1));
        assert_eq!(drained.next(), Some(2));
        assert_eq!(drained.next(), None);
    }
}
