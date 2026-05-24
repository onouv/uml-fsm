use super::{Control, Power};
pub struct SnubberNode {
    id: String,
    projected_load: Power,
    control: Control,
}

impl SnubberNode {
    pub fn new(id: String, projected_load: Power) -> Self {
        Self {
            id,
            projected_load,
            control: Control::default(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn projected_load(&self) -> Power {
        self.projected_load
    }
}
