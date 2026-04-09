#![allow(dead_code)] // This will bite me later. 25-11-24

mod errors;
mod nodes;
mod pages;
mod renderer;
mod storage;
mod style;
mod world;

use node_proc_macro::Node;
use storage::*;

#[derive(Clone, Node)]
struct Counter(u32);

#[derive(Clone, Node)]
struct Keyboard;

use crate::renderer::{rewrite, sub_win_rewrite};
/*
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

        let bx = Window {
            texts: vec![TextType::Text(Text::new(&count.0.to_string(), 1, 1, &[]))],
            width: 1,
            height: 1,
            type_of_border: TypeOfBorder::CurvedBorders,
        };

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
    let mut wow = [style::TextStyle::Blank; 12];
    wow[..slice.len()].copy_from_slice(slice);
    wow
}
*/
fn main() {
    let mut t = vec![];
    t.push(rewrite::Text::new_unchecked("Hi", 1, 10, &[]));
    t.push(rewrite::Text::new_unchecked("Hi", 1, 1, &[]));
    t.push(rewrite::Text::new_unchecked("Hi", 1, 5, &[]));
    // t.push(rewrite::Text::new_unchecked("Hi", 2, 1, &[]));
    // t.push(rewrite::Text::new_unchecked("Hi", 3, 1, &[]));
    // t.push(rewrite::Text::new_unchecked("Hi world", 3, 10, &[]));

    let win = rewrite::NonNestableWindow {
        texts: t,
        width: 52,
        height: 12,
        type_of_border: rewrite::TypeOfBorder::CurvedBorders,
    };

    let sub_window = sub_win_rewrite::SubWindow {
        window: win.clone().into(),
        line_number: 1,
        column: 1,
    };
    dbg!(sub_window.convert_to_texts().unwrap());
    let window = rewrite::NonNestableWindow {
        texts: sub_window.convert_to_texts().unwrap(),
        width: 100,
        height: 24,
        type_of_border: rewrite::TypeOfBorder::CurvedBorders,
    };
    
    println!("{}", window.render().unwrap());
}
