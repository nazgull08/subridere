use crate::click::OnClick;
use crate::drag::{DRAG_THRESHOLD, DragGhost, DragGhostStyle, DragState, Draggable, DropTarget, OnDragCancel, OnDragStart, OnDrop};
use crate::hover::{OnHover, OnHoverExit, OnPress};
use crate::right_click::OnRightClick;
use crate::visual::{Disabled, InteractiveVisual};
use crate::style::ButtonStyle;
use crate::tooltip::{Tooltip, TooltipState, TooltipStyle, TooltipUI};
use bevy::ui::FocusPolicy;
use bevy::window::Window;
use bevy::prelude::*;

/// Предыдущее состояние Interaction для отслеживания переходов
#[derive(Component, Default)]
pub struct PreviousInteraction(pub Interaction);

/// Система для OnClick — срабатывает при Pressed
pub(crate) fn handle_clicks(
    query: Query<(&Interaction, &OnClick), (Changed<Interaction>, Without<Disabled>)>,
    mut commands: Commands,
) {
    for (interaction, on_click) in &query {
        if *interaction == Interaction::Pressed {
            on_click.execute(&mut commands);
        }
    }
}

/// Система для OnRightClick — срабатывает при правом клике на hovered элементе
pub(crate) fn handle_right_clicks(
    query: Query<(&Interaction, &OnRightClick), Without<Disabled>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
) {
    // Правый клик только что нажат?
    if mouse.just_pressed(MouseButton::Right) {
        for (interaction, on_right_click) in &query {
            // Только если курсор над элементом
            if *interaction == Interaction::Hovered || *interaction == Interaction::Pressed {
                on_right_click.execute(&mut commands);
            }
        }
    }
}

/// Система для OnHover — срабатывает при входе в Hovered
pub(crate) fn handle_hover_actions(
    query: Query<(&Interaction, &OnHover), (Changed<Interaction>, Without<Disabled>)>,
    mut commands: Commands,
) {
    for (interaction, on_hover) in &query {
        if *interaction == Interaction::Hovered {
            on_hover.execute(&mut commands);
        }
    }
}

/// Система для OnHoverExit — срабатывает при выходе из Hovered
pub(crate) fn handle_hover_exit_actions(
    mut query: Query<(&Interaction, &mut PreviousInteraction, &OnHoverExit), Without<Disabled>>,
    mut commands: Commands,
) {
    for (interaction, mut prev, on_hover_exit) in &mut query {
        // Был Hovered, стал чем-то другим
        if prev.0 == Interaction::Hovered && *interaction != Interaction::Hovered {
            on_hover_exit.execute(&mut commands);
        }
        prev.0 = *interaction;
    }
}

/// Система для OnPress — срабатывает при нажатии
pub(crate) fn handle_press_actions(
    query: Query<(&Interaction, &OnPress), (Changed<Interaction>, Without<Disabled>)>,
    mut commands: Commands,
) {
    for (interaction, on_press) in &query {
        if *interaction == Interaction::Pressed {
            on_press.execute(&mut commands);
        }
    }
}

/// Система для визуального feedback
pub(crate) fn update_interactive_visuals(
    style: Res<ButtonStyle>,
    mut query: Query<
        (&Interaction, &mut BackgroundColor, Has<Disabled>),
        (Changed<Interaction>, With<InteractiveVisual>),
    >,
) {
    for (interaction, mut bg, is_disabled) in &mut query {
        *bg = BackgroundColor(if is_disabled {
            style.disabled
        } else {
            match interaction {
                Interaction::Pressed => style.pressed,
                Interaction::Hovered => style.hovered,
                Interaction::None => style.normal,
            }
        });
    }
}

pub(crate) fn handle_drag_start(
    query: Query<(Entity, &Interaction), (With<Draggable>, Without<Disabled>)>,
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    mut drag_state: ResMut<DragState>,
) {
    // Только если ещё не тащим и нажали кнопку
    if drag_state.dragging.is_some() || !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok(window) = windows.get_single() else {
        return;
    };
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    for (entity, interaction) in &query {
        if *interaction == Interaction::Hovered || *interaction == Interaction::Pressed {
            drag_state.dragging = Some(entity);
            drag_state.start_pos = cursor_pos;
            drag_state.drag_started = false;
            break;
        }
    }
}

pub(crate) fn handle_drag_move(
    draggable_query: Query<(Option<&OnDragStart>, &BackgroundColor), With<Draggable>>,
    windows: Query<&Window>,
    mut drag_state: ResMut<DragState>,
    ghost_style: Res<DragGhostStyle>,
    mut commands: Commands,
) {
    let Some(dragged_entity) = drag_state.dragging else {
        return;
    };

    if drag_state.drag_started {
        return;
    }

    let Ok(window) = windows.get_single() else {
        return;
    };
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    let distance = cursor_pos.distance(drag_state.start_pos);
    if distance < DRAG_THRESHOLD {
        return;
    }

    drag_state.drag_started = true;

    let ghost_color = if let Ok((_, bg_color)) = draggable_query.get(dragged_entity) {
        Color::srgba(
            bg_color.0.to_srgba().red,
            bg_color.0.to_srgba().green,
            bg_color.0.to_srgba().blue,
            ghost_style.opacity,
        )
    } else {
        ghost_style.background
    };

    // Ghost СПРАВА-СНИЗУ от курсора (смещение 15px)
    let offset = 15.0;
    let ghost = commands
        .spawn((
            DragGhost,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(cursor_pos.x + offset),
                top: Val::Px(cursor_pos.y + offset),
                width: Val::Px(ghost_style.size),
                height: Val::Px(ghost_style.size),
                ..default()
            },
            BackgroundColor(ghost_color),
            ZIndex(999),
        ))
        .id();

    drag_state.ghost_entity = Some(ghost);

    if let Ok((Some(on_drag_start), _)) = draggable_query.get(dragged_entity) {
        let action = on_drag_start.action.clone();
        commands.queue(move |world: &mut World| {
            action.execute(world);
        });
    }
}

/// Обновление позиции ghost
pub(crate) fn update_ghost_position(
    mut ghost_query: Query<&mut Node, With<DragGhost>>,
    windows: Query<&Window>,
    drag_state: Res<DragState>,
) {
    if !drag_state.drag_started {
        return;
    }

    let Some(ghost_entity) = drag_state.ghost_entity else {
        return;
    };

    let Ok(window) = windows.get_single() else {
        return;
    };
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    // То же смещение
    let offset = 15.0;
    if let Ok(mut node) = ghost_query.get_mut(ghost_entity) {
        node.left = Val::Px(cursor_pos.x + offset);
        node.top = Val::Px(cursor_pos.y + offset);
    }
}

/// Обработка drop или cancel
pub(crate) fn handle_drag_end(
    drop_target_query: Query<(Entity, &Interaction, Option<&OnDrop>), With<DropTarget>>,
    draggable_query: Query<(Option<&OnDragCancel>, &Interaction), With<Draggable>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut drag_state: ResMut<DragState>,
    mut commands: Commands,
) {
    if !drag_state.drag_started || !mouse.just_released(MouseButton::Left) {
        return;
    }

    let dragged_entity = drag_state.dragging;

    if let Some(ghost) = drag_state.ghost_entity {
        commands.entity(ghost).despawn_recursive();
    }

    // DEBUG: смотрим состояние всех элементов
    info!("=== DRAG END DEBUG ===");
    
    if let Some(entity) = dragged_entity {
        if let Ok((_, interaction)) = draggable_query.get(entity) {
            info!("Dragged element {:?} interaction: {:?}", entity, interaction);
        }
    }
    
    for (target_entity, interaction, _) in &drop_target_query {
        info!("DropTarget {:?} interaction: {:?}", target_entity, interaction);
    }

    // Ищем DropTarget который сейчас Hovered или Pressed
    let mut dropped_on_target = false;

    for (target_entity, interaction, on_drop) in &drop_target_query {
        if Some(target_entity) == dragged_entity {
            continue;
        }

        if *interaction == Interaction::Hovered || *interaction == Interaction::Pressed {
            info!("HIT on {:?}", target_entity);
            if let Some(on_drop) = on_drop {
                let action = on_drop.action.clone();
                commands.queue(move |world: &mut World| {
                    action.execute(world);
                });
            }
            dropped_on_target = true;
            break;
        }
    }

    if !dropped_on_target {
        info!("No target hit");
        if let Some(entity) = dragged_entity {
            if let Ok((Some(on_cancel), _)) = draggable_query.get(entity) {
                let action = on_cancel.action.clone();
                commands.queue(move |world: &mut World| {
                    action.execute(world);
                });
            }
        }
    }

    drag_state.clear();
}

/// Отмена drag если мышь отпущена до threshold
pub(crate) fn handle_drag_abort(
    mouse: Res<ButtonInput<MouseButton>>,
    mut drag_state: ResMut<DragState>,
) {
    if drag_state.dragging.is_some()
        && !drag_state.drag_started
        && mouse.just_released(MouseButton::Left)
    {
        drag_state.clear();
    }
}

/// Отслеживание hover над элементами с Tooltip
pub(crate) fn update_tooltip_hover(
    query: Query<(Entity, &Interaction, &Tooltip)>,
    mut tooltip_state: ResMut<TooltipState>,
    time: Res<Time>,
) {
    // Ищем элемент под курсором с Tooltip
    let mut found_hovered: Option<(Entity, &Tooltip)> = None;
    
    for (entity, interaction, tooltip) in &query {
        if *interaction == Interaction::Hovered || *interaction == Interaction::Pressed {
            found_hovered = Some((entity, tooltip));
            break;
        }
    }

    match found_hovered {
        Some((entity, _)) => {
            if tooltip_state.hovered == Some(entity) {
                // Продолжаем hover — увеличиваем таймер
                tooltip_state.hover_timer += time.delta_secs();
            } else {
                // Новый элемент — сбрасываем
                tooltip_state.hovered = Some(entity);
                tooltip_state.hover_timer = 0.0;
                tooltip_state.visible = false;
            }
        }
        None => {
            // Курсор не над tooltip элементом
            if tooltip_state.hovered.is_some() {
                tooltip_state.clear();
            }
        }
    }
}

/// Показ tooltip когда таймер превысил delay
pub(crate) fn show_tooltip(
    query: Query<&Tooltip>,
    mut tooltip_state: ResMut<TooltipState>,
    tooltip_style: Res<TooltipStyle>,
    windows: Query<&Window>,
    mut commands: Commands,
) {
    // Если уже показан или нечего показывать — выходим
    if tooltip_state.visible || tooltip_state.hovered.is_none() {
        return;
    }

    let entity = tooltip_state.hovered.unwrap();
    let Ok(tooltip) = query.get(entity) else {
        return;
    };

    // Проверяем таймер
    let delay_secs = tooltip.delay_ms as f32 / 1000.0;
    if tooltip_state.hover_timer < delay_secs {
        return;
    }

    // Получаем позицию курсора
    let Ok(window) = windows.single() else {
        return;
    };
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    // Создаём tooltip UI
    let tooltip_entity = commands
        .spawn((
            TooltipUI,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(cursor_pos.x + 10.0),
                top: Val::Px(cursor_pos.y + 10.0),
                padding: UiRect::all(Val::Px(tooltip_style.padding)),
                ..default()
            },
            BackgroundColor(tooltip_style.background),
            // Высокий z-index чтобы быть поверх всего
            ZIndex(1000),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(&tooltip.text),
                TextFont {
                    font_size: tooltip_style.font_size,
                    ..default()
                },
                TextColor(tooltip_style.text_color),
            ));
        })
        .id();

    tooltip_state.visible = true;
    tooltip_state.tooltip_entity = Some(tooltip_entity);
}

/// Скрытие tooltip когда курсор ушёл
pub(crate) fn hide_tooltip(
    mut tooltip_state: ResMut<TooltipState>,
    mut commands: Commands,
) {
    // Если tooltip показан, но hovered сброшен — удаляем
    if tooltip_state.tooltip_entity.is_some() && tooltip_state.hovered.is_none() {
        if let Some(entity) = tooltip_state.tooltip_entity.take() {
            commands.entity(entity).despawn();
        }
        tooltip_state.visible = false;
    }
}
