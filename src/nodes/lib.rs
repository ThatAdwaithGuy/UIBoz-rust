pub mod inputs;
pub mod label;
pub mod layout;
pub mod runtime;
pub mod widgets;
pub use crossterm::event::Event;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
