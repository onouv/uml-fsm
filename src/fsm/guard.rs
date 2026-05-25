pub trait Guard<S, Ev> {
    fn check(&self, state: &S, event: &Ev) -> bool;
}
