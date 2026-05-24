pub trait Action<R, E>
where
    E: std::error::Error,
{
    fn execute(&self) -> Result<R, E>;
}
