use crate::core::UiAction;
use crate::widgets::Disabled;
use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use std::sync::Arc;

/// Порог в пикселях для начала drag
const DRAG_THRESHOLD: f32 = 5.0;

/// Фаза drag'n'drop автомата
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DragPhase {
    #[default]
    Idle,
    Pending,
    Active,
}

/// Marker: элемент можно перетаскивать
#[derive(Component)]
pub struct Draggable;

/// Marker: элемент принимает drop
#[derive(Component)]
pub struct DropTarget;

/// Action при начале перетаскивания
#[derive(Component)]
pub struct OnDragStart {
    pub(crate) action: Arc<dyn UiAction>,
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
    pub(crate) action: Arc<dyn UiAction>,
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
    pub(crate) action: Arc<dyn UiAction>,
}

impl OnDragCancel {
    pub fn new(action: impl UiAction) -> Self {
        Self {
            action: Arc::new(action),
        }
    }
}

/// Состояние drag'n'drop
#[derive(Resource, Default)]
pub struct DragState {
    pub phase: DragPhase,
    pub dragging: Option<Entity>,
    pub drop_target: Option<Entity>,
    pub start_pos: Vec2,
    pub ghost_entity: Option<Entity>,
}

impl DragState {
    pub fn clear(&mut self) {
        self.phase = DragPhase::Idle;
        self.dragging = None;
        self.drop_target = None;
        self.start_pos = Vec2::ZERO;
        self.ghost_entity = None;
    }

    pub fn is_dragging(&self) -> bool {
        self.phase == DragPhase::Active
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

// ============ State Machine System ============

pub(crate) fn drag_system(
    mut drag_state: ResMut<DragState>,
    ghost_style: Res<DragGhostStyle>,
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    draggables: Query<
        (Entity, &Interaction, Option<&OnDragStart>, Option<&OnDragCancel>, Option<&BackgroundColor>),
        (With<Draggable>, Without<Disabled>),
    >,
    drop_targets: Query<(Entity, &Interaction, Option<&OnDrop>), With<DropTarget>>,
    mut ghost_query: Query<&mut Node, With<DragGhost>>,
    mut commands: Commands,
) {
    let cursor_pos = windows
        .single()
        .ok()
        .and_then(|w| w.cursor_position());

    match drag_state.phase {
        // ========== IDLE ==========
        DragPhase::Idle => {
            if !mouse.just_pressed(MouseButton::Left) {
                return;
            }

            let Some(cursor) = cursor_pos else { return };

            for (entity, interaction, _, _, _) in &draggables {
                if *interaction == Interaction::Hovered || *interaction == Interaction::Pressed {
                    drag_state.phase = DragPhase::Pending;
                    drag_state.dragging = Some(entity);
                    drag_state.start_pos = cursor;
                    return;
                }
            }
        }

        // ========== PENDING ==========
        DragPhase::Pending => {
            if mouse.just_released(MouseButton::Left) {
                drag_state.clear();
                return;
            }

            let Some(cursor) = cursor_pos else { return };
            let Some(dragged_entity) = drag_state.dragging else {
                drag_state.clear();
                return;
            };

            let distance = cursor.distance(drag_state.start_pos);
            if distance < DRAG_THRESHOLD {
                return;
            }

            drag_state.phase = DragPhase::Active;

            let ghost_color = draggables
                .get(dragged_entity)
                .ok()
                .and_then(|(_, _, _, _, bg)| bg)
                .map(|bg| {
                    let c = bg.0.to_srgba();
                    Color::srgba(c.red, c.green, c.blue, ghost_style.opacity)
                })
                .unwrap_or(ghost_style.background);

            let ghost = commands
                .spawn((
                    DragGhost,
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(cursor.x - ghost_style.size / 2.0),
                        top: Val::Px(cursor.y - ghost_style.size / 2.0),
                        width: Val::Px(ghost_style.size),
                        height: Val::Px(ghost_style.size),
                        ..default()
                    },
                    BackgroundColor(ghost_color),
                    ZIndex(999),
                    FocusPolicy::Pass,  // <-- Не блокирует hover!
                ))
                .id();

            drag_state.ghost_entity = Some(ghost);

            if let Ok((_, _, Some(on_start), _, _)) = draggables.get(dragged_entity) {
                let action = on_start.action.clone();
                commands.queue(move |world: &mut World| {
                    action.execute(world);
                });
            }
        }

        // ========== ACTIVE ==========
        DragPhase::Active => {
            let Some(dragged_entity) = drag_state.dragging else {
                drag_state.clear();
                return;
            };

            if let Some(cursor) = cursor_pos {
                if let Some(ghost_entity) = drag_state.ghost_entity {
                    if let Ok(mut node) = ghost_query.get_mut(ghost_entity) {
                        node.left = Val::Px(cursor.x - ghost_style.size / 2.0);
                        node.top = Val::Px(cursor.y - ghost_style.size / 2.0);
                    }
                }
            }

            if !mouse.just_released(MouseButton::Left) {
                return;
            }

            if let Some(ghost) = drag_state.ghost_entity.take() {
                commands.entity(ghost).despawn();
            }

            let mut found_target: Option<(Entity, Option<Arc<dyn UiAction>>)> = None;

            for (target_entity, interaction, on_drop) in &drop_targets {
                if target_entity == dragged_entity {
                    continue;
                }

                if *interaction == Interaction::Hovered || *interaction == Interaction::Pressed {
                    found_target = Some((target_entity, on_drop.map(|d| d.action.clone())));
                    break;
                }
            }

            if let Some((target_entity, action)) = found_target {
                drag_state.drop_target = Some(target_entity);

                if let Some(action) = action {
                    let dragging = drag_state.dragging;
                    let drop_target = drag_state.drop_target;
                    
                    commands.queue(move |world: &mut World| {
                        {
                            let mut state = world.resource_mut::<DragState>();
                            state.dragging = dragging;
                            state.drop_target = drop_target;
                        }
                        action.execute(world);
                    });
                }
            } else {
                if let Ok((_, _, _, Some(on_cancel), _)) = draggables.get(dragged_entity) {
                    let action = on_cancel.action.clone();
                    commands.queue(move |world: &mut World| {
                        action.execute(world);
                    });
                }
            }

            drag_state.clear();
        }
    }
}

/// Условие для запуска системы
pub fn has_draggables(query: Query<(), With<Draggable>>) -> bool {
    !query.is_empty()
}
