// src/game_init/state.rs
use bevy::prelude::*;

/// Этапы иниц
#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
#[states(scoped_entities)]
pub enum InitStage {
    #[default]
    Setup,
    AssetsLoading,
    MazeReady,
    LightsReady,
    PlayerReady,
    EnemiesReady,
    ItemsReady,
    Done,
}

pub trait Next {
    fn next(&self) -> Self;
}

impl Next for InitStage {
    fn next(&self) -> Self {
        match self {
            InitStage::Setup => InitStage::AssetsLoading,
            InitStage::AssetsLoading => InitStage::MazeReady,
            InitStage::MazeReady => InitStage::LightsReady,
            InitStage::LightsReady => InitStage::PlayerReady,
            InitStage::PlayerReady => InitStage::EnemiesReady,
            InitStage::EnemiesReady => InitStage::ItemsReady,
            InitStage::ItemsReady => InitStage::Done,
            InitStage::Done => InitStage::Done,
        }
    }
}
