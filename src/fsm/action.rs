pub trait Action<S, Evt, Err>
where
    Err: std::error::Error,
{
    fn execute(&self, state: &S, event: &Evt) -> Result<(), Err>;
}
