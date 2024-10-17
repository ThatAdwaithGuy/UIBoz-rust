#![feature(never_type)]
pub mod node;
pub use node::Node;
pub use node::{Immutable, Mutable, Storage};
use pages::PageManager;

pub struct App<V: ViewNode, C: ControllerNode> {
    view_node: V,
    controller_node: C,
    storage: Storage<Immutable>,
}

impl<V: ViewNode, C: ControllerNode> App<V, C> {
    pub fn new(view_node: V, controller_node: C, storage: Storage<Immutable>) -> Self {
        Self {
            view_node,
            controller_node,
            storage,
        }
    }

    pub fn run(&mut self) -> Option<()> {
        let is_running =  self.storage.get_mut::<Runtime>()?;
        let _ = self.storage.get_mut::<PageManager>()?;

        while is_running {
            print!("{}", self.view_node.view(&self.storage)?);
            self.controller_node.update(&self.storage);
        }

        Some(())
    }
}

pub trait ViewNode: Node {
    fn view(&self, storage: &Storage<Immutable>) -> Option<String>;
}

pub trait ControllerNode: Node {
    fn update(&mut self, storage: &Storage<Immutable>) -> Option<()>;
}

// *TEST*
#[derive(Clone)]
struct Counter(u32);
impl Node for Counter {}
#[derive(Clone)]
struct Keyboard;
impl Node for Keyboard {}
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};

impl Keyboard {
    fn getch(&self) -> std::io::Result<KeyCode> {
        enable_raw_mode()?;
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                disable_raw_mode()?;
                return Ok(key_event.code);
            }

            disable_raw_mode()?;
        }

        disable_raw_mode()?;
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Oops",
        ))
    }
}
#[derive(Clone)]
struct View;
impl Node for View {}
impl ViewNode for View {
    fn view(&self, storage: &Storage<Immutable>) -> Option<String> {
        let counter = storage.get::<Counter>()?;
        Some(format!("At {}\x1b[2H;H", counter.0))
    }
}

#[derive(Clone)]
struct Update;
impl Node for Update {}
impl ControllerNode for Update {
    fn update(&mut self, storage: &Storage<Immutable>) -> Option<()> {
        let mut counter = storage.get_mut::<Counter>()?;
        let keyboard = storage.get::<Keyboard>()?;
        match keyboard.getch().ok()? {
            KeyCode::Char('p') => counter.0 += 1,
            KeyCode::Char('m') => counter.0 -= 1,
            _ => {}
        }
        Some(())
    }
}
#[test]
fn feature() {
    let mut storage: Storage<Mutable> = Storage::new();
    let keyboard = Keyboard {};
    let mut counter = Counter(0);
    storage.put(keyboard);
    storage.put_mut(counter);
    let mut app = App {
        view_node: View {},
        controller_node: Update {},
        storage: storage.into_immutable(),
    };

    app.run();
}
