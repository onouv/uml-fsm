# uml-fsm

A small, type-driven finite state machine library for Rust.

## Core Ideas

### 1) Compile-time safety and correctness

State transitions are modeled with concrete Rust types and trait implementations.

- Each state is a distinct type.
- Each event is a distinct type.
- Only valid transitions exist in code, because you implement handlers only where allowed.

If a transition is not defined, it does not compile.

This gives a useful guarantee: the compiler enforces the transition graph you encoded.

### 2) `no_std` for embedded use

The library is `no_std` and depends on `core`, so it can run in embedded targets without the Rust standard library.

- No `std::error::Error` requirement.
- Infallible transition API.
- Guard outcomes (`Transitioned` vs `Stayed`) model flow control explicitly.

Examples in this repository may use `std` features (for example `println!`) for convenience, but the library itself is `no_std`.

### 3) Staying as close as reasonable to the UML 2 meta model

You should be able to match a UML state diagram by inspection to the implementation built with this lib. We only deviate to keep it simple enough for basic use. We will grow adaptation as we go. 

### 4) Synchronous State Transitions
The Machine emits Actions which may or may not be processed asynchronously  

## State Machine Core Model

The FSM is built from a few traits and types:

- `State`: state a system can be in. Provide optional lifecycle hooks (`on_enter`, `on_exit`) invoked around transitions (default no-op).
- `Event`: marker trait for transition triggers.
- `HandleEvent<Evt>`: transition logic for a `(State, Event)` pair.
- `Guard<S, Evt>`: decides whether a transition may fire.
- `Transition<S, Evt, Next, G, Eff>`: executes a guarded transition.
- `TransitionEffect<S, Evt>` models effect behavior when a transition actually happens
- `TransitionResult<Next, S>`: `Transitioned(Next)` or `Stayed(S)`.

## Integration Boundaries

### Direct Event Mode
Client code issues domain events into a serialized queue. A single dispatcher loop
applies them to the FSM core.

```text
User/UI/Adapter --> EventQueue<Evt> --> Single Dispatcher --> FSM Core
                                            |
                                            v
                                      Context updates
```

### Event / Action Mode
Client code issues domain events. The FSM core processes them synchronously,
updates context, and emits actions that represent required side effects.
`ActionRunner` processes actions and can push follow-up events into the same
event queue.

1. The context is the authoritative domain state data.
2. The state machine owns behavior rules and updates context synchronously.
3. Actions are effect intents emitted by the machine.
4. The `ActionRunner` executes effects and may feed follow-up events back.

This split keeps domain behavior deterministic while letting runtime concerns vary
across embedded and server environments.

```text

+--------------------------+
|  Clients                 |
+--------------------------+
      |            ^
      |            |
      v events     |
+--------------------------+
| EventQueue<Evt>          | optional follow-up events
| serialized ingress       |<-------------------------------+
+-------------+------------+                                |
              |                                             |
              v                                             |
      +-------+--------+                                    |
      | Single         |                                    |
      | Dispatcher     |                                    |
      +-------+--------+                                    |  
              |                                             |
              v                                             |
      +-------+------------------------------+              |
      | FSM Core (DispatchEvent<Evt, Act>)   |              |
      | - sync transition logic              |              |
      | - sync context updates               |              |
      +-------+------------------------------+              |
              |                                             |
              | emitted actions                             |
              v                                             |
      +-------+--------+      +----------------------+      |
      | ActionSink<Act>| ---> | ActionRunner<Act,Evt>|------+
      +----------------+      +----------+-----------+
```

To support embedded and server profiles with one core model, the library exposes
small event-first boundary traits:

- `DispatchEvent<Evt, Act>`: core event-in/action-out FSM API.
- `ActionRunner<Act, Evt>`: runtime adapter that executes actions and may emit follow-up events.
- `ActionSink<Act>`: action collector abstraction (fixed-capacity, dynamic, or no-op).
- `EventSink<Evt>` and `EventSource<Evt>`: queue boundaries for serialized event ingress.

Built-in helpers:

- `FixedActionSink<'a, Act>`: no-allocation sink backed by caller-provided storage.
- `NoActions`: drop all actions when a caller does not need side effects.
- `FixedEventQueue<'a, Evt>`: no-allocation FIFO for event sequencing.

## Features

Current Cargo features:

- `heapless`: enables adapters backed by `heapless` collections.
: exposes `HeaplessEventQueue<Evt, N>` and `HeaplessActionSink<Act, N>`.

By default, no optional features are enabled.

Enable in your `Cargo.toml`:

```toml
[dependencies]
uml-fsm = { version = "0.1.0", features = ["heapless"] }
```

Or from git:

```toml
[dependencies]
uml-fsm = { git = "https://github.com/<org>/<repo>", features = ["heapless"] }
```


## Examples

- `cargo run --example party-typestate`:    typestate-oriented explicit FSM flow.
- `cargo run --example party-runtime`:      event dispatch plus action runner feedback loop.




## Status

Early project, intentionally minimal.

