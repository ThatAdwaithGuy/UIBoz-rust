use std::{
    any::{Any, TypeId},
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    marker::PhantomData,
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
            Some(RefMut::map(cell.try_borrow_mut().ok()?, |boxed| {
                boxed.downcast_mut::<T>().expect("Downcast failed")
            }))
        })?
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
