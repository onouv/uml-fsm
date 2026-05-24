pub trait Guard {
    fn check(&self) -> bool;
}
