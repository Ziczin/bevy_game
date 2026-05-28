macro_rules! markers {
    { $($name:ident),+ $(,)? } => {
        $(
            #[derive(Component)]
            pub struct $name;
        )+
    };
}

macro_rules! animation_states {
    ($name:ident { $($field:ident),* $(,)? }) => {
        #[derive(Component, Default)]
        pub struct $name {
            $(pub $field: Handle<bevy_spritesheet_animation::prelude::Animation>,)*
        }
    };
}

#[doc(hidden)]
macro_rules! __resource_default {
    ($field:ident : $ty:ty = $default:expr) => {
        $field: $default
    };
    ($field:ident : $ty:ty) => {
        $field: <$ty>::default()
    };
}

macro_rules! component {
    ($name:ident { $($field:ident: $ty:ty),* $(,)? }) => {
        #[derive(::bevy::prelude::Component, ::std::default::Default)]
        pub struct $name {
            $(pub $field: $ty,)*
        }
    };
}

macro_rules! resource {
    ($name:ident { $($field:ident : $ty:ty $( = $default:expr )? ),* $(,)? }) => {
        #[derive(::bevy::prelude::Resource)]
        pub struct $name {
            $( pub $field: $ty, )*
        }
    };
}

macro_rules! behavior_states {
    ($name:ident { $first:ident $(, $rest:ident)* $(,)? }) => {
        ::paste::paste! {
            #[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
            pub enum [< $name State >] {
                #[default]
                $first,
                $( $rest ),*
            }

            #[derive(::bevy::prelude::Component)]
            pub struct [< $name StateHandler >] {
                pub current: [< $name State >],
                pub previous: [< $name State >],
            }

            impl Default for [< $name StateHandler >] {
                fn default() -> Self {
                    Self {
                        current: [< $name State >]::$first,
                        previous: [< $name State >]::$first,
                    }
                }
            }
            #[allow(dead_code)]
            impl [< $name StateHandler >] {
                pub fn new() -> Self {
                    Self::default()
                }

                pub fn set(&mut self, new_state: [< $name State >]) {
                    if self.previous != self.current {
                        self.previous = self.current;
                        self.current = new_state;
                    }
                }

                pub fn get(&self) -> [< $name State >] {
                    self.current
                }
            }
        }
    };
}

pub(crate) use markers;
pub(crate) use resource;
pub(crate) use component;
pub(crate) use behavior_states;
pub(crate) use animation_states;