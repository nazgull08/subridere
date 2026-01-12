use crate::click::OnClick;
use crate::drag::{Draggable, DropTarget, OnDragStart, OnDrop, OnDragCancel, DragState, DRAG_THRESHOLD};
use crate::hover::{OnHover, OnHoverExit, OnPress};
use crate::right_click::OnRightClick;
use crate::visual::{Disabled, InteractiveVisual};
use crate::style::ButtonStyle;
use crate::tooltip::{Tooltip, TooltipState, TooltipStyle, TooltipUI};
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

/// Начало потенциального drag (mouse down на Draggable)
pub(crate) fn handle_drag_start(
    query: Query<(Entity, &Interaction), (With<Draggable>, Without<Disabled>)>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut drag_state: ResMut<DragState>,
    windows: Query<&Window>,
) {
    // Начинаем только если ещё не тащим
    if drag_state.dragging.is_some() {
        return;
    }

    if mouse.just_pressed(MouseButton::Left) {
        // Ищем элемент под курсором
        for (entity, interaction) in &query {
            if *interaction == Interaction::Pressed {
                // Запоминаем, но drag ещё не начался
                drag_state.dragging = Some(entity);
                drag_state.drag_started = false;
                
                // Запоминаем позицию мыши
                if let Ok(window) = windows.single() {
                    if let Some(pos) = window.cursor_position() {
                        drag_state.start_pos = pos;
                    }
                }
                break;
            }
        }
    }
}

/// Отслеживание движения мыши во время drag
pub(crate) fn handle_drag_move(
    mut drag_state: ResMut<DragState>,
    windows: Query<&Window>,
    query: Query<&OnDragStart>,
    mut commands: Commands,
) {
    // Если есть потенциальный drag, но он ещё не начался
    if let Some(entity) = drag_state.dragging {
        if drag_state.drag_started {
            return; // Уже тащим
        }

        if let Ok(window) = windows.single() {
            if let Some(current_pos) = window.cursor_position() {
                let distance = current_pos.distance(drag_state.start_pos);
                
                // Превысили порог — drag начался
                if distance > DRAG_THRESHOLD {
                    drag_state.drag_started = true;
                    
                    // Вызываем OnDragStart если есть
                    if let Ok(on_drag_start) = query.get(entity) {
                        on_drag_start.execute(&mut commands);
                    }
                }
            }
        }
    }
}

/// Завершение drag (mouse up)
pub(crate) fn handle_drag_end(
    mut drag_state: ResMut<DragState>,
    mouse: Res<ButtonInput<MouseButton>>,
    drop_targets: Query<(Entity, &Interaction), With<DropTarget>>,
    on_drop_query: Query<&OnDrop>,
    on_cancel_query: Query<&OnDragCancel>,
    mut commands: Commands,
) {
    if mouse.just_released(MouseButton::Left) {
        if let Some(dragged_entity) = drag_state.dragging {
            if drag_state.drag_started {
                // Ищем DropTarget под курсором
                let mut dropped_on_target = false;
                
                for (target_entity, interaction) in &drop_targets {
                    if *interaction == Interaction::Hovered || *interaction == Interaction::Pressed {
                        // Нашли target — вызываем OnDrop
                        if let Ok(on_drop) = on_drop_query.get(target_entity) {
                            on_drop.execute(&mut commands);
                            dropped_on_target = true;
                        }
                        break;
                    }
                }
                
                // Если не попали на target — вызываем OnDragCancel
                if !dropped_on_target {
                    if let Ok(on_cancel) = on_cancel_query.get(dragged_entity) {
                        on_cancel.execute(&mut commands);
                    }
                }
            }
            
            // Очищаем состояние
            drag_state.clear();
        }
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
