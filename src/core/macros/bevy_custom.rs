#[macro_export]
macro_rules! markers {
    { $($name:ident),+ $(,)? } => {
        $(
            #[derive(bevy::prelude::Component)]
            pub struct $name;
        )+
    };
}

#[macro_export]
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

#[macro_export]
macro_rules! bevy_flags_inner {
    ($shift:expr, $first:ident) => {
        const $first = 1 << $shift;
    };
    ($shift:expr, $first:ident, $($rest:ident),+ $(,)?) => {
        const $first = 1 << $shift;
        bevy_flags_inner!($shift + 1, $($rest),+);
    };
}

#[macro_export]
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

#[macro_export]
macro_rules! flags_for {
    { $name:ident, [$f1:ident, $f2:ident, $f3:ident, $f4:ident, $f5:ident, $f6:ident, $f7:ident, $f8:ident, $f9:ident, $f10:ident, $f11:ident, $f12:ident, $f13:ident, $f14:ident, $f15:ident, $f16:ident, $f17:ident, $f18:ident, $f19:ident, $f20:ident, $f21:ident, $f22:ident, $f23:ident, $f24:ident, $f25:ident, $f26:ident, $f27:ident, $f28:ident, $f29:ident, $f30:ident, $f31:ident, $f32:ident, $f33:ident, $f34:ident, $f35:ident, $f36:ident, $f37:ident, $f38:ident, $f39:ident, $f40:ident, $f41:ident, $f42:ident, $f43:ident, $f44:ident, $f45:ident, $f46:ident, $f47:ident, $f48:ident, $f49:ident, $f50:ident, $f51:ident, $f52:ident, $f53:ident, $f54:ident, $f55:ident, $f56:ident, $f57:ident, $f58:ident, $f59:ident, $f60:ident, $f61:ident, $f62:ident, $f63:ident, $f64:ident, $f65:ident $(, $rest:ident)*] } => {
        bevy_flags_impl!($name, u128, 0, $f1, $f2, $f3, $f4, $f5, $f6, $f7, $f8, $f9, $f10, $f11, $f12, $f13, $f14, $f15, $f16, $f17, $f18, $f19, $f20, $f21, $f22, $f23, $f24, $f25, $f26, $f27, $f28, $f29, $f30, $f31, $f32, $f33, $f34, $f35, $f36, $f37, $f38, $f39, $f40, $f41, $f42, $f43, $f44, $f45, $f46, $f47, $f48, $f49, $f50, $f51, $f52, $f53, $f54, $f55, $f56, $f57, $f58, $f59, $f60, $f61, $f62, $f63, $f64, $f65 $(, $rest)*);
    };
    { $name:ident, [$f1:ident, $f2:ident, $f3:ident, $f4:ident, $f5:ident, $f6:ident, $f7:ident, $f8:ident, $f9:ident, $f10:ident, $f11:ident, $f12:ident, $f13:ident, $f14:ident, $f15:ident, $f16:ident, $f17:ident, $f18:ident, $f19:ident, $f20:ident, $f21:ident, $f22:ident, $f23:ident, $f24:ident, $f25:ident, $f26:ident, $f27:ident, $f28:ident, $f29:ident, $f30:ident, $f31:ident, $f32:ident, $f33:ident $(, $rest:ident)*] } => {
        bevy_flags_impl!($name, u64, 0, $f1, $f2, $f3, $f4, $f5, $f6, $f7, $f8, $f9, $f10, $f11, $f12, $f13, $f14, $f15, $f16, $f17, $f18, $f19, $f20, $f21, $f22, $f23, $f24, $f25, $f26, $f27, $f28, $f29, $f30, $f31, $f32, $f33 $(, $rest)*);
    };
    { $name:ident, [$f1:ident, $f2:ident, $f3:ident, $f4:ident, $f5:ident, $f6:ident, $f7:ident, $f8:ident, $f9:ident, $f10:ident, $f11:ident, $f12:ident, $f13:ident, $f14:ident, $f15:ident, $f16:ident, $f17:ident $(, $rest:ident)*] } => {
        bevy_flags_impl!($name, u32, 0, $f1, $f2, $f3, $f4, $f5, $f6, $f7, $f8, $f9, $f10, $f11, $f12, $f13, $f14, $f15, $f16, $f17 $(, $rest)*);
    };
    { $name:ident, [$f1:ident, $f2:ident, $f3:ident, $f4:ident, $f5:ident, $f6:ident, $f7:ident, $f8:ident, $f9:ident $(, $rest:ident)*] } => {
        bevy_flags_impl!($name, u16, 0, $f1, $f2, $f3, $f4, $f5, $f6, $f7, $f8, $f9 $(, $rest)*);
    };
    { $name:ident, [$($flags:ident),* $(,)?] } => {
        bevy_flags_impl!($name, u8, 0, $($flags),*);
    };
}