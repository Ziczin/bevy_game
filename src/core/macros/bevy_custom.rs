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

pub(crate) use {
    markers,
    animation_states,
};