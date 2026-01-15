use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SystemMenuState {
    #[default]
    Closed,
    Open,
}

pub fn system_menu_closed(state: Res<State<SystemMenuState>>) -> bool {
    *state.get() == SystemMenuState::Closed
}

pub fn system_menu_open(state: Res<State<SystemMenuState>>) -> bool {
    *state.get() == SystemMenuState::Open
}
