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

macro_rules! component_from_type {
    ($name:ident, $type:ty) => {
        #[derive(Component, Default, Debug, PartialEq, Clone, Copy)]
        #[repr(transparent)]
        pub struct $name(pub $type);
        
        impl std::ops::Deref for $name {
            type Target = $type;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        
        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
        
        impl From<$type> for $name {
            fn from(value: $type) -> Self {
                Self(value)
            }
        }
        
        impl From<$name> for $type {
            fn from(value: $name) -> Self {
                value.0
            }
        }
    };
}

#[allow(unused_imports)]
pub(crate) use {
    component,
    resource,
    component_from_type,
};