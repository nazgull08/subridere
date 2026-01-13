use bevy::prelude::*;

use crate::DragState;

/// Компонент для элементов, которые показывают tooltip при наведении
#[derive(Component)]
pub struct Tooltip {
    pub text: String,
    pub delay_ms: u32,
}

impl Tooltip {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            delay_ms: 500,
        }
    }

    pub fn with_delay(text: impl Into<String>, delay_ms: u32) -> Self {
        Self {
            text: text.into(),
            delay_ms,
        }
    }
}

/// Состояние системы tooltip
#[derive(Resource, Default)]
pub struct TooltipState {
    pub hovered: Option<Entity>,
    pub hover_timer: f32,
    pub visible: bool,
    pub tooltip_entity: Option<Entity>,
}

impl TooltipState {
    pub fn clear(&mut self) {
        self.hovered = None;
        self.hover_timer = 0.0;
        self.visible = false;
    }
}

/// Marker для tooltip UI элемента
#[derive(Component)]
pub struct TooltipUI;

/// Стиль tooltip
#[derive(Resource)]
pub struct TooltipStyle {
    pub background: Color,
    pub text_color: Color,
    pub font_size: f32,
    pub padding: f32,
}

impl Default for TooltipStyle {
    fn default() -> Self {
        Self {
            background: Color::srgba(0.1, 0.1, 0.1, 0.95),
            text_color: Color::srgb(0.9, 0.9, 0.9),
            font_size: 14.0,
            padding: 8.0,
        }
    }
}

// ============ Systems ============

/// Отслеживание hover над элементами с Tooltip
pub(crate) fn update_tooltip_hover(
    query: Query<(Entity, &Interaction, &Tooltip)>,
    mut tooltip_state: ResMut<TooltipState>,
    time: Res<Time>,
) {
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
                tooltip_state.hover_timer += time.delta_secs();
            } else {
                tooltip_state.hovered = Some(entity);
                tooltip_state.hover_timer = 0.0;
                tooltip_state.visible = false;
            }
        }
        None => {
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
    if tooltip_state.visible || tooltip_state.hovered.is_none() {
        return;
    }

    let entity = tooltip_state.hovered.unwrap();
    let Ok(tooltip) = query.get(entity) else {
        return;
    };

    let delay_secs = tooltip.delay_ms as f32 / 1000.0;
    if tooltip_state.hover_timer < delay_secs {
        return;
    }

    let Ok(window) = windows.single() else {
        return;
    };
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

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

pub(crate) fn hide_tooltip(
    mut tooltip_state: ResMut<TooltipState>,
    drag_state: Res<DragState>,
    mut commands: Commands,
) {
    // Скрываем если: курсор ушёл ИЛИ идёт drag
    let should_hide = tooltip_state.hovered.is_none() || drag_state.is_dragging();

    if tooltip_state.tooltip_entity.is_some() && should_hide {
        if let Some(entity) = tooltip_state.tooltip_entity.take() {
            commands.entity(entity).despawn();
        }
        tooltip_state.visible = false;
        tooltip_state.hovered = None; // Сбрасываем полностью при drag
    }
}
