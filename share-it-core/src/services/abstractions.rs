pub trait Handles<T> {
    type Result;

    fn handle(&mut self, cmd: T) -> Self::Result;
}
