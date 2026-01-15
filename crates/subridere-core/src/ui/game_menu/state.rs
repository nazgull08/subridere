use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameMenuState {
    #[default]
    Closed,
    Open,
}

pub fn game_menu_closed(state: Res<State<GameMenuState>>) -> bool {
    *state.get() == GameMenuState::Closed
}

pub fn game_menu_open(state: Res<State<GameMenuState>>) -> bool {
    *state.get() == GameMenuState::Open
}
