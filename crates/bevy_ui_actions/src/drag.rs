use crate::action::UiAction;
use crate::visual::Disabled;
use bevy::prelude::*;
use std::sync::Arc;

/// Порог в пикселях для начала drag
pub const DRAG_THRESHOLD: f32 = 5.0;

/// Marker: элемент можно перетаскивать
#[derive(Component)]
pub struct Draggable;

/// Marker: элемент принимает drop
#[derive(Component)]
pub struct DropTarget;

/// Action при начале перетаскивания
#[derive(Component)]
pub struct OnDragStart {
    pub action: Arc<dyn UiAction>,
}

impl OnDragStart {
    pub fn new(action: impl UiAction) -> Self {
        Self {
            action: Arc::new(action),
        }
    }
}

/// Action при успешном drop на target
#[derive(Component)]
pub struct OnDrop {
    pub action: Arc<dyn UiAction>,
}

impl OnDrop {
    pub fn new(action: impl UiAction) -> Self {
        Self {
            action: Arc::new(action),
        }
    }
}

/// Action при отмене drag (отпустили не на target)
#[derive(Component)]
pub struct OnDragCancel {
    pub action: Arc<dyn UiAction>,
}

impl OnDragCancel {
    pub fn new(action: impl UiAction) -> Self {
        Self {
            action: Arc::new(action),
        }
    }
}

/// Состояние drag & drop
#[derive(Resource, Default)]
pub struct DragState {
    /// Entity который тащим
    pub dragging: Option<Entity>,
    /// Начальная позиция курсора
    pub start_pos: Vec2,
    /// Drag уже начался (превысили threshold)
    pub drag_started: bool,
    /// Entity ghost элемента
    pub ghost_entity: Option<Entity>,
}

impl DragState {
    pub fn clear(&mut self) {
        self.dragging = None;
        self.start_pos = Vec2::ZERO;
        self.drag_started = false;
        self.ghost_entity = None;
    }
}

/// Marker для ghost UI
#[derive(Component)]
pub struct DragGhost;

/// Стиль ghost
#[derive(Resource)]
pub struct DragGhostStyle {
    pub background: Color,
    pub size: f32,
    pub opacity: f32,
}

impl Default for DragGhostStyle {
    fn default() -> Self {
        Self {
            background: Color::srgba(0.5, 0.5, 0.8, 0.7),
            size: 50.0,
            opacity: 0.7,
        }
    }
}
