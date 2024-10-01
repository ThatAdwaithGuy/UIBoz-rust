macro_rules! is_trait {
    ($name:ty, $trait_name:path) => {{
        trait __InnerMarkerTrait {
            fn __is_trait_inner_method() -> bool {
                false
            }
        }
        struct __TraitTest<T>(T);
        impl<T: $trait_name> __TraitTest<T> {
            fn __is_trait_inner_method() -> bool {
                true
            }
        }
        impl<T> __InnerMarkerTrait for __TraitTest<T> {}
        __TraitTest::<$name>::__is_trait_inner_method()
    }};
}

macro_rules! impl_node {
    ($type:ty) => {
        impl node::Node for $type {}
    };
}

pub(crate) use impl_node;

pub(crate) use is_trait;
