use crate::action::UiAction;
use bevy::prelude::*;
use std::sync::Arc;

/// Marker: этот элемент можно перетаскивать
#[derive(Component)]
pub struct Draggable;

/// Marker: на этот элемент можно бросить
#[derive(Component)]
pub struct DropTarget;

/// Действие когда начали тащить этот элемент
#[derive(Component)]
pub struct OnDragStart {
    action: Arc<dyn UiAction>,
}

impl OnDragStart {
    pub fn new(action: impl UiAction) -> Self {
        Self {
            action: Arc::new(action),
        }
    }

    pub(crate) fn execute(&self, commands: &mut Commands) {
        let action = self.action.clone();
        commands.queue(move |world: &mut World| {
            action.execute(world);
        });
    }
}

/// Действие когда что-то бросили НА этот элемент
#[derive(Component)]
pub struct OnDrop {
    action: Arc<dyn UiAction>,
}

impl OnDrop {
    pub fn new(action: impl UiAction) -> Self {
        Self {
            action: Arc::new(action),
        }
    }

    pub(crate) fn execute(&self, commands: &mut Commands) {
        let action = self.action.clone();
        commands.queue(move |world: &mut World| {
            action.execute(world);
        });
    }
}

/// Действие когда drag отменён (бросили не на target)
#[derive(Component)]
pub struct OnDragCancel {
    action: Arc<dyn UiAction>,
}

impl OnDragCancel {
    pub fn new(action: impl UiAction) -> Self {
        Self {
            action: Arc::new(action),
        }
    }

    pub(crate) fn execute(&self, commands: &mut Commands) {
        let action = self.action.clone();
        commands.queue(move |world: &mut World| {
            action.execute(world);
        });
    }
}

/// Текущее состояние drag & drop
#[derive(Resource, Default)]
pub struct DragState {
    /// Entity которую тащим
    pub dragging: Option<Entity>,
    /// Начальная позиция мыши
    pub start_pos: Vec2,
    /// Drag уже начался (мышь сдвинулась)
    pub drag_started: bool,
}

impl DragState {
    /// Есть ли активный drag
    pub fn is_dragging(&self) -> bool {
        self.dragging.is_some() && self.drag_started
    }

    /// Очистить состояние
    pub fn clear(&mut self) {
        self.dragging = None;
        self.drag_started = false;
    }
}

/// Порог в пикселях для начала drag (чтобы случайно не начать при клике)
pub const DRAG_THRESHOLD: f32 = 5.0;
