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
            }

            impl Default for [< $name StateHandler >] {
                fn default() -> Self {
                    Self {
                        current: [< $name State >]::$first,
                    }
                }
            }
            #[allow(dead_code)]
            impl [< $name StateHandler >] {
                pub fn new() -> Self {
                    Self::default()
                }

                pub fn set(&mut self, new_state: [< $name State >]) -> bool {
                    if self.current != new_state {
                        self.current = new_state;
                        return true;
                    }
                    return false;
                }

                pub fn get(&self) -> [< $name State >] {
                    self.current
                }
            }
        }
    };
}

pub(crate) use behavior_states;