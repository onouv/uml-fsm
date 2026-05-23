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
}
