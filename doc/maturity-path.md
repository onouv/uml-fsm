
1. v1: Single generic Transition struct
- Keep one Transition artifact as a struct with compile-time generics.
- Keep State, Guard, Action as traits.
- Return a small enum for outcome: Stayed or Moved.
- Use this while your transition kinds all share one shape.

2. v2: Add a domain state enum
- Keep Transition as struct.
- Introduce one domain enum that represents all runtime states.
- Centralize state switching and event handling around that enum.
- Move here when you need clearer orchestration and fewer generic types leaking outward.

3. v3: Add transition enum variants only when needed
- Introduce a TransitionKind enum (for example Internal, External, Timed, Choice) only if transition families start differing in data or rules.
- Keep the outer API stable; hide variant-specific complexity inside matching logic.
- Move here when you see repeated optional fields or branching constructors.

4. v4: Introduce a trait for transition engines/policies
- Add a trait only if you truly need multiple interchangeable execution policies.
- Example use cases: simulation mode, audited mode, real-time mode.
- Avoid this until you have at least two real implementations.

Decision checklist:
1. Do I have one shape of transition data? Use struct.
2. Do I need “one of many transition kinds”? Add enum.
3. Do I need interchangeable implementations behind one API? Add trait.
4. Am I solving a current problem, not a possible future? If no, stay at current version.

If you want, I can draft a minimal v1 API surface (types + method signatures) that is easy to grow into v2 without breaking callers.