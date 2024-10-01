pub mod macros;
pub mod node;
use node::Node;

trait ViewNode: node::Node {
    fn view(&self, node_container: &node::NodeStorage) -> Option<String>;
}

trait ControllerNode: node::Node {
    fn mutate(&mut self, node_container: &mut node::NodeStorage);
}

struct App<V: ViewNode, M: ControllerNode> {
    view: V,
    controller: M,
    nodes: node::NodeStorage,
}

impl<V: ViewNode, M: ControllerNode> App<V, M> {
    fn new(view: V, model: M) -> App<V, M> {
        Self {
            view,
            controller: model,
            nodes: node::NodeStorage::new(),
        }
    }

    fn run(&mut self, refresh_screen: bool) -> Option<()> {
        loop {
            println!("{}", self.view.view(&self.nodes)?);
            if refresh_screen {
                print!("\x1b[2J");
                print!("\x1b[H");
            }
            self.controller.mutate(&mut self.nodes);
        }
    }
}
struct Counter {
    counter: usize,
}
impl Node for Counter {}
struct View {}
impl Node for View {}

impl ViewNode for View {
    fn view(&self, node_container: &node::NodeStorage) -> Option<String> {
        let string = format!(
            "Hello, World for {}",
            node_container.get::<Counter>()?.counter
        );
        Some(string.to_string())
    }
}

struct Rollers {}
macros::impl_node!(Rollers);
impl ControllerNode for Rollers {
    fn mutate(&mut self, node_container: &mut node::NodeStorage) {
        let mut counter = node_container.get_mut::<Counter>().unwrap();
        counter.counter += 1;
    }
}

#[test]
fn feature() {
    let mut node_container = node::NodeStorage::new();
    let mut counter = Counter { counter: 0 };
    node_container.put(counter);
    let mut app: App<View, Rollers> = App {
        view: View {},
        controller: Rollers {},
        nodes: node_container,
    };
    app.run(true);
}

struct DefaultKeyboardInput {}
macros::impl_node!(DefaultKeyboardInput);
