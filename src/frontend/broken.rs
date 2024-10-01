pub mod macros;
pub mod node;
use std::any::Any;

use node::*;
//trait Model {
//    fn view(&self) -> String;
//    fn controller(&mut self, event_listener: &NodeContainer);
//}
#[derive(Debug, Clone)]
struct Counter {
    count: u32,
}

impl Node for Counter {}

impl Counter {
    fn new() -> Self {
        Self { count: 0 }
    }
}

fn counter_check(data: &dyn Any) -> bool {
    data.is::<Counter>()
}

struct MutableCounter(Counter);
impl Node for MutableCounter {}
impl MutableCounter {
    fn new() -> Self {
        MutableCounter(Counter::new())
    }
}

fn mutable_counter_check(data: &dyn Any) -> bool {
    data.is::<MutableCounter>()
}
#[cfg(test)]
mod tests {
    use std::{
        any::{Any, TypeId},
        boxed::ThinBox,
        mem::transmute,
        rc::Rc,
    };

    use crate::is_trait;

    use super::*;

    #[test]
    fn feature() {
        let mut node_container = NodeContainer::new();
        let mut vtable = Vtable::new();

        vtable.add_new_check(counter_check);
        let counter = Counter::new();

        node_container.put(ThinBox::new(counter));

        for x in node_container.thin_values {
            for check in vtable.checking_functions.clone().into_iter() {
                if check(&x) {
                    dbg!(&x);
                }
            }
        }
    }

    #[test]
    fn test_is_trait_macro() {
        assert!(is_trait!(Counter, Node));
    }
}
