#[macro_export]
macro_rules! component {
    ($name:ident { $($field:ident: $ty:ty),* $(,)? }) => {
        #[derive(::bevy::prelude::Component, ::std::default::Default)]
        pub struct $name {
            $(pub $field: $ty,)*
        }
    };
}

#[macro_export]
macro_rules! resource {
    ($name:ident { $($field:ident : $ty:ty $( = $default:expr )? ),* $(,)? }) => {
        #[derive(::bevy::prelude::Resource)]
        pub struct $name {
            $( pub $field: $ty, )*
        }
    };
}