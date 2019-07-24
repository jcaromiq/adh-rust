
pub trait Command {
    fn new() -> Self;
    fn execute(&self);
}