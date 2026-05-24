use thiserror::Error;
use uml_fsm::fsm::State;

pub struct Gathering {
    origin: String,
    attendees: Vec<String>,
}
pub struct Traveling {
    origin: String,
    destination: String,
    attendees: Vec<String>,
}

pub struct Arrived {
    destination: String,
    attendees: Vec<String>,
}

pub enum Party {
    Gathering(Gathering),
    Traveling(Traveling),
    Arrived(Arrived),
}

#[derive(Debug, Error)]
pub enum PartyError {
    #[error("Cannot start traveling without attendees")]
    NoAttendees,
}

impl Party {
    fn new(origin: String) -> Self {
        Self::Gathering {
            origin,
            attendees: Vec::new(),
        }
    }
}

impl State<PartyError> for Gathering {
    fn on_enter(&self) -> Result<(), PartyError> {
        println!("Party is assembling at {}", self.origin);
        Ok(())
    }

    fn on_exit(&self) -> Result<(), PartyError> {
        if self.attendees.is_empty() {
            return Err(PartyError::NoAttendees);
        }

        println!(
            "Party has assembled {:?} at: {:?}",
            self.origin, self.attendees
        );
        Ok(())
    }
}
