use bevy::prelude::*;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameStates {
    #[default]
    Explore,
    PhoneOpen,
}
