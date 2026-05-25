pub trait State {
    fn on_enter(&self) {}
    fn on_exit(&self) {}
}
