use crate::{errors::TextError, sub_win};
use dyn_clone::DynClone;
use std::collections::HashMap;

/*
AIM:
```rust
fn main() {
    App::new()
       .add_node(..)
       .run();
```

enum View {
    Str(String),
    Window(Window),
}

trait Node {
    // Updates the app According to the user. Usually have key bindings and other stuff.
    fn update(&mut self, app: &App);
    // Returns a View to be printed
    fn view(&self) -> Result<View, ...>;
    // Get the app ID
    fn get_app_id(&self) -> u32;
    // Other functions
}

struct App {
    nodes: HashMap<u32, Box<dyn Node>>,
    // Other fields
}

impl App {
    fn new() -> App {
        App {
            nodes: HashMap::new()}
        }
    }
    fn add_node(&mut self, node: Box<dyn Node>) -> Self
}

*/
#[derive(Debug, Clone)]
struct App {
    nodes: HashMap<u32, Box<dyn Node>>,
    // Other fields
}

impl App {
    fn new() -> App {
        App {
            nodes: HashMap::new(),
        }
    }
    fn add_node(&mut self, node: Box<dyn Node>) -> Self {
        self.nodes.insert(node.get_app_id(), node);
        self.to_owned()
    }
}
enum View {
    Str(String),
    Window(sub_win::SubWindow),
}

trait Node: DynClone {
    // Updates the app According to the user. Usually have key bindings and other stuff.
    fn update(&mut self, app: &App);
    // Returns a View to be printed
    fn view(&self) -> Result<View, TextError>;
    // Get the app ID
    fn get_app_id(&self) -> u32;
    // Other functions
}
