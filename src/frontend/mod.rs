use std::{
    any::{Any, TypeId},
    collections::HashMap,
};
pub mod macros;
pub mod node;
use std::io;
use std::{thread, time};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{self, is_tty};

use node::*;
//trait Model {
//    fn view(&self) -> String;
//    fn controller(&mut self, event_listener: &NodeContainer);
//}
#[derive(Debug)]
struct Counter {
    count: u32,
}

impl Node for Counter {}

impl Counter {
    fn new() -> Self {
        Self { count: 0 }
    }
}

struct MutableCounter(Counter);
impl Node for MutableCounter {}
impl MutableCounter {
    fn new() -> Self {
        MutableCounter(Counter::new())
    }
}

#[cfg(test)]
mod tests {
    use crate::is_trait;

    use super::*;

    #[test]
    fn feature() {
        let mut node_container = NodeContainer::new();
        let counter = Counter::new();
        let mut mut_counter = MutableCounter::new();

        node_container.put(counter);
        node_container.put_mut(&mut mut_counter);

        //for (type_id, is_mutable) in node_container.events {
        //    match is_mutable {
        //        IsMutable::Mutable(boxed_node) => {
        //            let node = boxed_node.downcast_ref().unwrap();
        //        }
        //        IsMutable::NonMutable(boxed_node) => todo!(),
        //    }
        //}
    }

    #[test]
    fn test_is_trait_macro() {
        assert!(is_trait!(Counter, Node));
    }
}
