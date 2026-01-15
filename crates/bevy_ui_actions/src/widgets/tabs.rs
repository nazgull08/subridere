use bevy::prelude::*;

use super::Active;

/// Группа вкладок — хранит индекс активной
#[derive(Component, Default)]
pub struct TabGroup {
    pub active: usize,
}

impl TabGroup {
    pub fn new(active: usize) -> Self {
        Self { active }
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

/// Система: клик по Tab меняет TabGroup.active
/// Ищет TabGroup вверх по иерархии
pub(crate) fn handle_tab_clicks(
    tab_query: Query<(Entity, &Interaction, &Tab, &ChildOf), Changed<Interaction>>,
    parent_query: Query<&ChildOf>,
    mut group_query: Query<&mut TabGroup>,
) {
    for (_, interaction, tab, parent) in &tab_query {
        if *interaction != Interaction::Pressed {
            continue;
        }

        // Ищем TabGroup вверх по иерархии
        let mut current = parent.parent();

        for _ in 0..10 {
            if let Ok(mut group) = group_query.get_mut(current) {
                if group.active != tab.index {
                    group.active = tab.index;
                }
                break;
            }

            if let Ok(next_parent) = parent_query.get(current) {
                current = next_parent.parent();
            } else {
                break;
            }
        }
    }
}

/// Система: обновление видимости TabContent
/// Использует Display::None чтобы скрытый контент не занимал место
pub(crate) fn sync_tab_content_visibility(
    group_query: Query<(Entity, &TabGroup), Changed<TabGroup>>,
    children_query: Query<&Children>,
    mut content_query: Query<(&TabContent, &mut Node)>,
) {
    for (group_entity, group) in &group_query {
        let mut to_visit = vec![group_entity];

        while let Some(entity) = to_visit.pop() {
            if let Ok((content, mut node)) = content_query.get_mut(entity) {
                node.display = if content.index == group.active {
                    Display::Flex
                } else {
                    Display::None
                };
            }

            if let Ok(children) = children_query.get(entity) {
                to_visit.extend(children.iter());
            }
        }
    }
}

/// Система: маркер Active на активной вкладке
/// Ищет Tab рекурсивно вниз
pub(crate) fn sync_active_tab_marker(
    group_query: Query<(Entity, &TabGroup), Changed<TabGroup>>,
    children_query: Query<&Children>,
    tab_query: Query<&Tab>,
    mut commands: Commands,
) {
    for (group_entity, group) in &group_query {
        let mut to_visit = vec![group_entity];

        while let Some(entity) = to_visit.pop() {
            if let Ok(tab) = tab_query.get(entity) {
                if tab.index == group.active {
                    commands.entity(entity).insert(Active);
                } else {
                    commands.entity(entity).remove::<Active>();
                }
            }

            if let Ok(children) = children_query.get(entity) {
                to_visit.extend(children.iter());
            }
        }
    }
}
