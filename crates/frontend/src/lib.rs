#![feature(never_type)]

use std::{
    any::{Any, TypeId},
    borrow::Borrow,
    cell::{Ref, RefCell, RefMut},
    collections::{btree_map::Values, HashMap},
    hash::Hash,
    marker::PhantomData,
    rc::Rc,
};

pub trait Node {}

pub struct Mutable;
#[derive(Debug)]
pub struct Immutable;
#[derive(Debug)]
pub struct Storage<S = Immutable> {
    //nodes: HashMap<TypeId, Box<dyn Any>>,
    nodes: HashMap<TypeId, RefCell<Box<dyn Any>>>,
    marker: PhantomData<S>,
}

impl Storage<Immutable> {
    pub fn get<T: Node + 'static>(&self) -> Option<Ref<T>> {
        let cell = self.nodes.get(&TypeId::of::<T>())?;

        // Borrow the RefCell, returning a Ref<Box<dyn Any>>
        let borrow = cell.borrow();

        // Use Ref::map to create a Ref<T> from the Ref<Box<dyn Any>>
        Ref::filter_map(borrow, |boxed| boxed.downcast_ref::<T>()).ok()
    }

    pub fn get_mut<T: Node + 'static>(&self) -> Option<RefMut<T>> {
        self.nodes.get(&TypeId::of::<T>()).map(|cell| {
            RefMut::map(cell.borrow_mut(), |boxed| {
                boxed.downcast_mut::<T>().expect("Downcast failed")
            })
        })
    }
}

impl Storage<Mutable> {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            marker: PhantomData,
        }
    }
    pub fn into_immutable(self) -> Storage<Immutable> {
        let mut immutable_storage = Storage::<Immutable> {
            nodes: HashMap::new(),
            marker: PhantomData,
        };

        // Transfer immutable nodes
        for (type_id, boxed_node) in self.nodes {
            immutable_storage.nodes.insert(type_id, boxed_node);
        }

        immutable_storage
    }

    pub fn put<T: Node + 'static>(&mut self, node: T) {
        self.nodes
            .insert(TypeId::of::<T>(), RefCell::new(Box::new(node)));
    }

    pub fn get<T: Node + 'static>(&self) -> Option<Ref<T>> {
        let cell = self.nodes.get(&TypeId::of::<T>())?;

        let borrow = cell.borrow();

        // Use Ref to create a Ref<T> from the Ref<Box<dyn Any>>
        Ref::filter_map(borrow, |boxed| boxed.downcast_ref::<T>()).ok()
    }

    pub fn remove<T: Node + 'static>(&mut self) {
        self.nodes.remove(&TypeId::of::<T>());
    }

    pub fn put_mut<T: Node + 'static>(&mut self, node: T) {
        self.nodes
            .insert(TypeId::of::<T>(), RefCell::new(Box::new(node)));
    }

    pub fn get_mut<T: Node + 'static>(&self) -> Option<RefMut<T>> {
        // Get the node
        self.nodes.get(&TypeId::of::<T>()).map(|cell| {
            // Pass the borrow checker with this sorcery
            RefMut::map(cell.borrow_mut(), |boxed| {
                // TODO: remove this expect.
                boxed.downcast_mut::<T>().expect("Downcast failed")
            })
        })
    }

    pub fn remove_mut<T: Node + 'static>(&mut self) {
        self.nodes.remove(&TypeId::of::<T>());
    }
}

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
        let is_running = match self.storage.get_mut::<Runtime>() {
            Some(x) => x.is_running,
            None => return None,
        };

        while is_running {
            print!("{}", self.view_node.view(&self.storage)?);
            self.controller_node.update(&self.storage);
        }

        Some(())
    }
}

pub struct Runtime {
    pub is_running: bool,
}
impl Node for Runtime {}
// *TEST*
pub trait ViewNode: Node {
    fn view(&self, storage: &Storage<Immutable>) -> Option<String>;
}

pub trait ControllerNode: Node {
    fn update(&mut self, storage: &Storage<Immutable>) -> Option<()>;
}

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
