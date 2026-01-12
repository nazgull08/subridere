use bevy::prelude::*;

/// Компонент для элементов, которые показывают tooltip при наведении
#[derive(Component)]
pub struct Tooltip {
    /// Текст tooltip
    pub text: String,
    /// Задержка перед показом (миллисекунды)
    pub delay_ms: u32,
}

impl Tooltip {
    /// Создать tooltip с текстом и дефолтной задержкой (500ms)
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            delay_ms: 500,
        }
    }

    /// Создать tooltip с кастомной задержкой
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
    /// Entity над которым сейчас курсор (с Tooltip)
    pub hovered: Option<Entity>,
    /// Сколько времени курсор над элементом
    pub hover_timer: f32,
    /// Tooltip сейчас показан?
    pub visible: bool,
    /// Entity самого tooltip UI (для удаления)
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
