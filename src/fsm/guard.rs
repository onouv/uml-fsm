pub trait Guard<S, Evt> {
    fn check(&self, state: &S, event: &Evt) -> bool;
}
