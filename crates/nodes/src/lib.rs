pub mod inputs;
pub mod label;
pub mod widgets;
pub mod runtime;

pub use crossterm::event::Event;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
