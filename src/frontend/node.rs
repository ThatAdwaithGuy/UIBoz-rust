use std::{
    any::{Any, TypeId},
    collections::HashMap,
    ops::DerefMut,
    rc::Rc,
};

pub trait Node {}
pub enum IsMutable<'a> {
    Mutable(Box<&'a mut dyn Any>),
    NonMutable(Box<dyn Any>),
}

pub struct NodeContainer<'a> {
    pub events: HashMap<TypeId, IsMutable<'a>>,
}

impl<'a> NodeContainer<'a> {
    pub fn new() -> Self {
        NodeContainer {
            events: HashMap::new(),
        }
    }

    pub fn get_all_values(&self) -> Vec<&Box<dyn Any>> {
        self.events
            .keys()
            .map(|val| {
                self.events
                    .get(val)
                    .map(|val| match val {
                        IsMutable::Mutable(x) => x.downcast_ref().unwrap(),
                        IsMutable::NonMutable(x) => x.downcast_ref().unwrap(),
                    })
                    .unwrap()
            })
            .collect()
    }

    // Thanks to NotAPenguin. https://notapenguin0.github.io/posts/rust-event-systems/
    pub fn put<T: 'static + Node>(&mut self, item: T) {
        self.events
            .insert(TypeId::of::<T>(), IsMutable::NonMutable(Box::new(item)));
    }

    pub fn put_mut<T: 'static + Node>(&mut self, item: &'a mut T) {
        self.events
            .insert(TypeId::of::<T>(), IsMutable::Mutable(Box::new(item)));
    }

    // You can get a mutable as a immutable reference but you cannot get a immutable
    // as a mutable reference.
    pub fn get<T: 'static + Node>(&self) -> Option<&T> {
        let any = self.events.get(&TypeId::of::<T>());
        any.map(|val| match val {
            IsMutable::Mutable(x) => x.downcast_ref().unwrap(),
            IsMutable::NonMutable(x) => x.downcast_ref().unwrap(),
        })
    }

    pub fn get_mut<T: 'static + Node>(&mut self) -> Option<&mut T> {
        self.events
            .get_mut(&TypeId::of::<T>())
            .and_then(|val| match val {
                IsMutable::Mutable(x) => x.downcast_mut(),
                IsMutable::NonMutable(_) => panic!("ERROR"),
            })
    }
}
