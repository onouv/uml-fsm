pub struct Party {
    attendees: Vec<String>,
    origin: String,
    destination: Option<String>,
}

impl Party {
    pub fn new(origin: String) -> Self {
        Self {
            attendees: vec![],
            origin,
            destination: None,
        }
    }

    pub fn attendees(&self) -> &[String] {
        &self.attendees
    }

    pub fn origin(&self) -> &str {
        &self.origin
    }

    pub fn destination(&self) -> Option<&str> {
        self.destination.as_deref()
    }

    pub fn add_member(&mut self, member: String) {
        self.attendees.push(member);
    }

    pub fn remove_member(&mut self, member: &str) {
        self.attendees.retain(|m| m != member);
    }

    pub fn can_depart(&self) -> bool {
        !self.attendees.is_empty()
    }

    pub fn set_destination(&mut self, destination: String) {
        self.destination = Some(destination);
    }

    pub fn disband(&mut self) {
        self.attendees.clear();
        self.destination = None;
    }
}
