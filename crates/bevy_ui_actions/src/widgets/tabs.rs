use bevy::prelude::*;

/// Группа вкладок — хранит индекс активной
#[derive(Component)]
pub struct TabGroup {
    pub active: usize,
}

impl TabGroup {
    pub fn new(active: usize) -> Self {
        Self { active }
    }
}

impl Default for TabGroup {
    fn default() -> Self {
        Self { active: 0 }
    }
}

/// Кнопка-вкладка
#[derive(Component)]
pub struct Tab {
    pub index: usize,
}

impl Tab {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
}

/// Контент вкладки — показывается когда tab.index == group.active
#[derive(Component)]
pub struct TabContent {
    pub index: usize,
}

impl TabContent {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
}

/// Маркер для активной вкладки (добавляется/убирается автоматически)
#[derive(Component)]
pub struct ActiveTab;

/// Система: клик по Tab меняет TabGroup.active
pub(crate) fn handle_tab_clicks(
    tab_query: Query<(&Interaction, &Tab, &ChildOf), Changed<Interaction>>,
    mut group_query: Query<&mut TabGroup>,
) {
    for (interaction, tab, parent) in &tab_query {
        if *interaction == Interaction::Pressed {
            if let Ok(mut group) = group_query.get_mut(parent.get()) {
                if group.active != tab.index {
                    group.active = tab.index;
                }
            }
        }
    }
}

/// Система: обновление видимости TabContent
pub(crate) fn sync_tab_content_visibility(
    group_query: Query<(&TabGroup, &Children), Changed<TabGroup>>,
    mut content_query: Query<(&TabContent, &mut Visibility)>,
) {
    for (group, children) in &group_query {
        for child in children.iter() {
            if let Ok((content, mut visibility)) = content_query.get_mut(child) {
                *visibility = if content.index == group.active {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                };
            }
        }
    }
}

/// Система: маркер ActiveTab на активной вкладке
pub(crate) fn sync_active_tab_marker(
    group_query: Query<(&TabGroup, &Children), Changed<TabGroup>>,
    tab_query: Query<&Tab>,
    mut commands: Commands,
) {
    for (group, children) in &group_query {
        for child in children.iter() {
            if let Ok(tab) = tab_query.get(child) {
                if tab.index == group.active {
                    commands.entity(child).insert(ActiveTab);
                } else {
                    commands.entity(child).remove::<ActiveTab>();
                }
            }
        }
    }
}
