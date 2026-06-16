macro_rules! component {
    ($name:ident { $($field:ident: $ty:ty),* $(,)? }) => {
        #[derive(::bevy::prelude::Component, ::std::default::Default)]
        pub struct $name {
            $(pub $field: $ty,)*
        }
    };
}

#[allow(unused_macros)]
macro_rules! resource {
    ($name:ident { $($field:ident : $ty:ty $( = $default:expr )? ),* $(,)? }) => {
        #[derive(::bevy::prelude::Resource)]
        pub struct $name {
            $( pub $field: $ty, )*
        }
    };
}

#[allow(unused_imports)]
pub(crate) use {
    component,
    resource
};