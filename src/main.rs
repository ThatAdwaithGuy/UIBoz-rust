#![feature(thin_box)]

mod errors;
mod nodes;
mod pages;
mod renderer;
mod storage;
mod style;
mod world;

use node_proc_macro::Node;
use nodes::runtime::Runtime;
use storage::*;
use world::ControllerNode;
use world::ViewNode;

#[derive(Clone, Node)]
struct Counter(u32);

#[derive(Clone, Node)]
struct Keyboard;

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

impl Keyboard {
    fn getch(&self) -> std::io::Result<event::KeyEvent> {
        enable_raw_mode()?;
        if event::poll(std::time::Duration::from_millis(1000))? {
            if let Event::Key(key) = event::read()? {
                disable_raw_mode()?;
                return Ok(key);
            } else {
                disable_raw_mode()?;
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Invalid event captured",
                ));
            }
        } else {
            disable_raw_mode()?;
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid event captured",
            ));
        }
    }
}

#[derive(Clone, Node)]
struct View;
use renderer::*;
impl ViewNode for View {
    fn view(&self, storage: &Storage<Immutable>) -> Option<String> {
        let count = storage.get_mut::<Counter>()?;
        dbg!(count.0);

        let bx = Window::new(
            vec![TextType::Text(Text::new(
                &count.0.to_string(),
                1,
                1,
                &[],
            ))],
            1,
            1,
            TypeOfBorder::CurvedBorders,
        );

        Some(format!("\x1b[2J\x1b[H{}\n", count.0))
    }
}

#[derive(Clone, Node)]
struct Update;

impl ControllerNode for Update {
    fn update(&mut self, storage: &Storage<Immutable>) -> Option<()> {
        let mut counter = storage.get_mut::<Counter>().unwrap();
        let keyboard = storage.get::<Keyboard>().unwrap();
        let mut run_time = storage.get_mut::<Runtime>().unwrap();
        match keyboard.getch() {
            Ok(option) => match option.code {
                KeyCode::Char('p') => counter.0 += 1,
                KeyCode::Char('m') => counter.0 -= 1,
                KeyCode::Char('q') => {
                    run_time.is_running = false;
                }
                _ => {}
            },
            Err(_) => {}
        }

        Some(())
    }
}

fn fill(slice: &[style::TextStyle]) -> [style::TextStyle; 12] {
    let mut wow = [style::TextStyle::Bold(false); 12];
    wow[..slice.len()].copy_from_slice(slice);
    wow
}

fn main() {}
