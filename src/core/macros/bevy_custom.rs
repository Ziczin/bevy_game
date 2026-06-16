macro_rules! markers {
    ($($name:ident),+ $(,)?) => {
        $(
            #[derive(bevy::prelude::Component)]
            pub struct $name;
        )+
    };
}

macro_rules! animation_states {
    ($name:ident { $($field:ident),* $(,)? }) => {
        ::paste::paste! {
            #[derive(bevy::prelude::Component, Default)]
            pub struct [< $name Animation >] {
                $(pub $field: bevy::prelude::Handle<bevy_spritesheet_animation::prelude::Animation>,)*
            }
        }
    };
}

#[allow(unused_macros)]
macro_rules! bevy_flags_inner {
    ($shift:expr, $first:ident $(, $rest:ident)*) => {
        const $first = 1 << $shift;
        $(
            bevy_flags_inner!($shift + 1, $rest);
        )*
    };
}

#[allow(unused_macros)]
macro_rules! bevy_flags_impl {
    ($name:ident, $type:ty, $shift:expr, $($flags:ident),+ $(,)?) => {
        bitflags::bitflags! {
            #[derive(bevy::prelude::Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct $name: $type {
                bevy_flags_inner!($shift, $($flags),+);
            }
        }
    };
}

#[allow(unused_macros)]
macro_rules! flags_for {
    ($name:ident, $type:ty, $($flags:ident),+ $(,)?) => {
        bevy_flags_impl!($name, $type, 0, $($flags),+);
    };
}

pub(crate) use {
    markers,
    animation_states,
    flags_for
};