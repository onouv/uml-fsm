pub struct Transition<R, E> 
    where E: std::error::Error
{
    from: State,
    to: State,
    guard: Guard,
    action: Action<R, E>,
}

impl Transition<R, E> {

    pub fn new(from: State, to: State, guard: Guard, action: Action<R, E>) -> Self {
        Self { from, to, guard, action }
    }

    pub fn on_trigger<R>(&self) -> Result<State,E> {
        // Check guard condition
        if self.guard.check() {
            self.from.on_exit(|| {
                // Optional: perform any exit actions here
            });
            // Execute action
            self.action.execute<R>();
            
            // Return the next state
            self.to
        } else {
            // If guard condition fails, stay in the current state
            self.from
        }
    }
}

pub trait Action<R, E> 
    where E: std::error::Error
{
    fn execute(&self) -> Result<R, E>;
}

pub trait Guard {
    fn check(&self) -> bool;
}

pub trait State<E> 
    where E: std::error::Error
{
    fn on_enter(&self, f: FnOnce) -> Result<Self, E>;
    fn on_exit(&self, f: FnOnce) -> Result<Self, E>;
}
