use pages::PageManager;
pub use storage::Node;
pub use storage::{Immutable, Mutable, Storage};

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
        let is_running = self.storage.get_mut::<nodes::runtime::Runtime>()?;
        let _ = self.storage.get_mut::<PageManager>()?;

        while is_running.is_running {
            print!("{}", self.view_node.view(&self.storage).unwrap());
            self.controller_node.update(&self.storage).unwrap();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
