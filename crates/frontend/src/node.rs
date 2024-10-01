#![feature(thin_box)]

use std::{
    any::{Any, TypeId},
    boxed::ThinBox,
    collections::HashMap,
    marker::{PhantomData, Unsize},
    mem::transmute,
    ops::{Deref, DerefMut},
};

pub trait Node {}

#[derive(Debug, Default)]
pub struct NodeStorage {
    dyn_items: HashMap<TypeId, ThinBox<dyn Any>>,
    items: HashMap<TypeId, Box<dyn Any>>,
}

impl NodeStorage {
    /// Create a new registry
    pub fn new() -> Self {
        Self {
            dyn_items: HashMap::default(),
            items: HashMap::default(),
        }
    }

    // thanks to NotAPenguin0, who wrote this code

    fn put_dyn_boxed<T: ?Sized + 'static + Node>(&mut self, item: ThinBox<T>) {
        // SAFETY: ThinBox always has the same size regardless of the type inside,
        // so we can transmute this to a different pointer until we cast it back to
        // T in get()
        let any = unsafe { std::mem::transmute::<_, ThinBox<dyn Any>>(item) };
        self.dyn_items.insert(TypeId::of::<T>(), any);
    }

    /// Put a static type `T` into the registry. This can then be retrieved back
    /// by calling [`Self::get::<T>()`]
    pub fn put<T: 'static + Node>(&mut self, item: T) {
        self.items.insert(TypeId::of::<T>(), Box::new(item));
    }

    /// Put a trait object into the registry. If called with `dyn MyTrait`, this takes in
    /// any `Foo: MyTrait`, which is then moved into the registry and can be queried back with
    /// [`Self::get_dyn::<dyn MyTrait>()`]
    pub fn put_dyn<T: ?Sized + 'static + Node>(&mut self, item: impl Unsize<T>) {
        self.put_dyn_boxed(ThinBox::<T>::new_unsize(item));
    }

    /// Get the registered object for `T`, or `None` if it didn't exist.
    pub fn get<T: 'static + Node>(&self) -> Option<&T> {
        let any = self.items.get(&TypeId::of::<T>());
        any.map(|value| value.downcast_ref::<T>().unwrap())
    }

    /// Get a mutable reference to the registered object for `T`, or `None` if it didn't exist.
    pub fn get_mut<T: 'static + Node>(&mut self) -> Option<&mut T> {
        let any = self.items.get_mut(&TypeId::of::<T>());
        any.map(|value| value.downcast_mut::<T>().unwrap())
    }

    /// Get the registered implementation for `dyn MyTrait`, or `None` if it didn't exist.
    pub fn get_dyn<T: ?Sized + 'static + Node>(&self) -> Option<&T> {
        let any = self.dyn_items.get(&TypeId::of::<T>());
        any.map(|any| unsafe { std::mem::transmute::<_, &ThinBox<T>>(any) }.deref())
    }

    /// Get a mutable reference to the registered implementation for `dyn MyTrait`, or `None` if it didn't exist.
    pub fn get_dyn_mut<T: ?Sized + 'static + Node>(&mut self) -> Option<&mut T> {
        let any = self.dyn_items.get_mut(&TypeId::of::<T>());
        any.map(|any| unsafe { std::mem::transmute::<_, &mut ThinBox<T>>(any) }.deref_mut())
    }
}
