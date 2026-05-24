struct Operating;
struct JoiningSwarm;
struct InSwarm;
struct Alone;

enum SwarmState {
    Operating(Operating),
    JoiningSwarm(JoiningSwarm),
    InSwarm(InSwarm),
    Alone(Alone),
}

pub(crate) struct Control;

impl Default for Control {
    fn default() -> Self {
        Self
    }
}

impl Control {
    pub fn run() -> ! {
        loop {}
    }
}
