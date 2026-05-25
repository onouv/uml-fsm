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

You should be able to match a UML state diagram to the implementation build with this lib. We only deviate to keep it simple enough for basic use. We will grow adaptation as we go. 

## Minimal Model

The FSM is built from a few traits and types:

- `State`: optional lifecycle hooks (`on_enter`, `on_exit`) invoked around transitions (default no-op).
- `Event`: marker trait for transition triggers.
- `HandleEvent<Evt>`: transition logic for a `(State, Event)` pair.
- `Guard<S, Evt>`: decides whether a transition may fire.
- `Transition<S, Evt, Next, G, Eff>`: executes a guarded transition.
- `TransitionEffect<S, Evt>` models effect behavior when a transition actually happens
- `TransitionResult<Next, S>`: `Transitioned(Next)` or `Stayed(S)`.

## Why this design

- Invalid transitions are compile-time errors.
- Blocked transitions are runtime values (`Stayed`), not exceptions.
- The API stays small and explicit, which fits embedded and safety-focused code.

## Status

Early project, intentionally minimal.

Current focus:

- Keep the core API small and predictable.
- Preserve `no_std` compatibility.
- Add examples that show typed transition flows.