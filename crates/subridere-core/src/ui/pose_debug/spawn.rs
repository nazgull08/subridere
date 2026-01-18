// crates/subridere-core/src/ui/pose_debug/spawn.rs

use bevy::prelude::*;
use bevy_ui_actions::prelude::*;

use crate::player::arm::pose_debug::{DebugPose, EditTarget, PoseDebugState};

use super::components::*;

// ═══════════════════════════════════════════════════════════════════
// COLORS
// ═══════════════════════════════════════════════════════════════════

const PANEL_BG: Color = Color::srgba(0.08, 0.08, 0.10, 0.95);
const PANEL_BORDER: Color = Color::srgb(0.3, 0.5, 0.3);
const BUTTON_BG: Color = Color::srgb(0.15, 0.18, 0.15);
const BUTTON_BG_ACTIVE: Color = Color::srgb(0.2, 0.35, 0.2);
const TEXT_COLOR: Color = Color::srgb(0.85, 0.9, 0.85);
const TEXT_DIM: Color = Color::srgb(0.5, 0.55, 0.5);
const LABEL_COLOR: Color = Color::srgb(0.6, 0.7, 0.6);

// ═══════════════════════════════════════════════════════════════════
// SPAWN
// ═══════════════════════════════════════════════════════════════════

pub fn spawn_pose_debug_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/dogica.ttf");

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(10.0),
                top: Val::Px(10.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(15.0)),
                row_gap: Val::Px(10.0),
                border: UiRect::all(Val::Px(2.0)),
                min_width: Val::Px(280.0),
                ..default()
            },
            BackgroundColor(PANEL_BG),
            BorderColor(PANEL_BORDER),
            GlobalZIndex(300),
            PoseDebugRoot,
            Name::new("Pose Debug Panel"),
        ))
        .with_children(|panel| {
            // ═══════════════════════════════════════════════════════
            // HEADER
            // ═══════════════════════════════════════════════════════
            spawn_header(panel, &font);
            spawn_separator(panel);

            // ═══════════════════════════════════════════════════════
            // POSE SELECTOR
            // ═══════════════════════════════════════════════════════
            spawn_row_with_nav(
                panel,
                &font,
                "Pose:",
                PoseNameText,
                "sword_windup",
                PrevPoseAction,
                NextPoseAction,
            );
            // ═══════════════════════════════════════════════════════
            // EDIT TARGET SELECTOR
            // ═══════════════════════════════════════════════════════
            spawn_row_with_button(
                panel,
                &font,
                "Editing:",
                EditTargetText,
                "hand_offset",
                NextEditTargetAction,
            );
            // ═══════════════════════════════════════════════════════
            // AXIS SELECTOR
            // ═══════════════════════════════════════════════════════
            spawn_row_with_nav(
                panel,
                &font,
                "Axis:",
                AxisText,
                "Z (roll)",
                PrevAxisAction,
                NextAxisAction,
            );
            spawn_separator(panel);
            // ═══════════════════════════════════════════════════════
            // VALUE ADJUSTER
            // ═══════════════════════════════════════════════════════
            spawn_value_adjuster(panel, &font);
            spawn_separator(panel);
            // ═══════════════════════════════════════════════════════
            // CURRENT VALUES DISPLAY
            // ═══════════════════════════════════════════════════════
            panel.spawn((
                Text::new("hand_offset: (0.00, 0.00, 0.00)\nelbow_hint: (0.00, 0.00, 0.00)\nrotation: (0.0, 0.0, 0.0)"),
                TextFont {
                    font: font.clone(),
                    font_size: 10.0,
                    ..default()
                },
                TextColor(TEXT_DIM),
                PoseValuesText,
            ));
            spawn_separator(panel);
            // ═══════════════════════════════════════════════════════
            // ACTIONS
            // ═══════════════════════════════════════════════════════
            panel
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(8.0),
                    justify_content: JustifyContent::Center,
                    ..default()
                })
                .with_children(|row| {
                    spawn_action_button(row, &font, "Print All", PrintAllPosesAction);
                    spawn_action_button(row, &font, "Close", ClosePoseDebugAction);
                });
        });

    info!("🎨 Pose Debug UI spawned");
}

pub fn despawn_pose_debug_ui(mut commands: Commands, query: Query<Entity, With<PoseDebugRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
    info!("🎨 Pose Debug UI despawned");
}

// ═══════════════════════════════════════════════════════════════════
// UI HELPERS
// ═══════════════════════════════════════════════════════════════════

fn spawn_header(parent: &mut ChildSpawnerCommands, font: &Handle<Font>) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|row| {
            row.spawn((
                Text::new("🎨 POSE DEBUG"),
                TextFont {
                    font: font.clone(),
                    font_size: 14.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
            ));
        });
}

fn spawn_separator(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(1.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
    ));
}

fn spawn_row_with_nav<M: Component>(
    parent: &mut ChildSpawnerCommands,
    font: &Handle<Font>,
    label: &str,
    marker: M,
    initial_value: &str,
    prev_action: impl UiAction,
    next_action: impl UiAction,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|row| {
            // Label
            row.spawn((
                Text::new(label),
                TextFont {
                    font: font.clone(),
                    font_size: 11.0,
                    ..default()
                },
                TextColor(LABEL_COLOR),
            ));

            // Nav buttons + value
            row.spawn(Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(5.0),
                ..default()
            })
            .with_children(|nav| {
                spawn_small_button(nav, font, "◀", prev_action);

                nav.spawn((
                    Text::new(initial_value),
                    TextFont {
                        font: font.clone(),
                        font_size: 10.0,
                        ..default()
                    },
                    TextColor(TEXT_COLOR),
                    Node {
                        min_width: Val::Px(100.0),
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    marker,
                ));

                spawn_small_button(nav, font, "▶", next_action);
            });
        });
}

fn spawn_row_with_button<M: Component>(
    parent: &mut ChildSpawnerCommands,
    font: &Handle<Font>,
    label: &str,
    marker: M,
    initial_value: &str,
    action: impl UiAction,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|row| {
            row.spawn((
                Text::new(label),
                TextFont {
                    font: font.clone(),
                    font_size: 11.0,
                    ..default()
                },
                TextColor(LABEL_COLOR),
            ));

            row.spawn((
                Button,
                Node {
                    padding: UiRect::axes(Val::Px(10.0), Val::Px(4.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(BUTTON_BG),
                OnClick::new(action),
                InteractiveVisual,
            ))
            .with_children(|btn| {
                btn.spawn((
                    Text::new(initial_value),
                    TextFont {
                        font: font.clone(),
                        font_size: 10.0,
                        ..default()
                    },
                    TextColor(TEXT_COLOR),
                    marker,
                ));
            });
        });
}

fn spawn_value_adjuster(parent: &mut ChildSpawnerCommands, font: &Handle<Font>) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(5.0),
            ..default()
        })
        .with_children(|col| {
            // Current value display
            col.spawn((
                Text::new("0.00"),
                TextFont {
                    font: font.clone(),
                    font_size: 18.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
                CurrentValueText,
            ));

            // Buttons row
            col.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(5.0),
                ..default()
            })
            .with_children(|row| {
                spawn_adjust_button(row, font, "--", AdjustValueAction { delta: -0.15 });
                spawn_adjust_button(row, font, "-", AdjustValueAction { delta: -0.05 });
                spawn_adjust_button(row, font, "+", AdjustValueAction { delta: 0.05 });
                spawn_adjust_button(row, font, "++", AdjustValueAction { delta: 0.15 });
            });

            // Rotation specific (bigger steps)
            col.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(5.0),
                ..default()
            })
            .with_children(|row| {
                spawn_adjust_button(row, font, "-15°", AdjustValueAction { delta: -15.0 });
                spawn_adjust_button(row, font, "-5°", AdjustValueAction { delta: -5.0 });
                spawn_adjust_button(row, font, "+5°", AdjustValueAction { delta: 5.0 });
                spawn_adjust_button(row, font, "+15°", AdjustValueAction { delta: 15.0 });
            });
        });
}

fn spawn_small_button(
    parent: &mut ChildSpawnerCommands,
    font: &Handle<Font>,
    label: &str,
    action: impl UiAction,
) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(24.0),
                height: Val::Px(24.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(BUTTON_BG),
            OnClick::new(action),
            InteractiveVisual,
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new(label),
                TextFont {
                    font: font.clone(),
                    font_size: 12.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
            ));
        });
}

fn spawn_adjust_button(
    parent: &mut ChildSpawnerCommands,
    font: &Handle<Font>,
    label: &str,
    action: impl UiAction,
) {
    parent
        .spawn((
            Button,
            Node {
                padding: UiRect::axes(Val::Px(8.0), Val::Px(4.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(BUTTON_BG),
            OnClick::new(action),
            InteractiveVisual,
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new(label),
                TextFont {
                    font: font.clone(),
                    font_size: 10.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
            ));
        });
}

fn spawn_action_button(
    parent: &mut ChildSpawnerCommands,
    font: &Handle<Font>,
    label: &str,
    action: impl UiAction,
) {
    parent
        .spawn((
            Button,
            Node {
                padding: UiRect::axes(Val::Px(12.0), Val::Px(6.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(BUTTON_BG),
            OnClick::new(action),
            InteractiveVisual,
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new(label),
                TextFont {
                    font: font.clone(),
                    font_size: 11.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
            ));
        });
}

// ═══════════════════════════════════════════════════════════════════
// ACTIONS
// ═══════════════════════════════════════════════════════════════════

struct NextPoseAction;
impl UiAction for NextPoseAction {
    fn execute(&self, world: &mut World) {
        let mut state = world.resource_mut::<PoseDebugState>();
        state.current_pose = state.current_pose.next();
        info!("🎨 Pose: {}", state.current_pose.name());
    }
}

struct PrevPoseAction;
impl UiAction for PrevPoseAction {
    fn execute(&self, world: &mut World) {
        let mut state = world.resource_mut::<PoseDebugState>();
        state.current_pose = state.current_pose.prev();
        info!("🎨 Pose: {}", state.current_pose.name());
    }
}

struct NextEditTargetAction;
impl UiAction for NextEditTargetAction {
    fn execute(&self, world: &mut World) {
        let mut state = world.resource_mut::<PoseDebugState>();
        state.edit_target = state.edit_target.next();
        info!("🎨 Editing: {}", state.edit_target.name());
    }
}

struct NextAxisAction;
impl UiAction for NextAxisAction {
    fn execute(&self, world: &mut World) {
        let mut state = world.resource_mut::<PoseDebugState>();
        state.axis = (state.axis + 1) % 3;
    }
}

struct PrevAxisAction;
impl UiAction for PrevAxisAction {
    fn execute(&self, world: &mut World) {
        let mut state = world.resource_mut::<PoseDebugState>();
        state.axis = (state.axis + 2) % 3;
    }
}

struct AdjustValueAction {
    delta: f32,
}
impl UiAction for AdjustValueAction {
    fn execute(&self, world: &mut World) {
        let mut state = world.resource_mut::<PoseDebugState>();

        // Копируем нужные значения до mutable borrow
        let current_pose = state.current_pose;
        let edit_target = state.edit_target;
        let axis = state.axis;

        // Определяем шаг в зависимости от edit_target
        let actual_delta = match edit_target {
            EditTarget::HandRotation => self.delta, // Для rotation используем как есть (градусы)
            _ => {
                // Для offset/elbow конвертируем: ±15 → ±0.15, ±5 → ±0.05
                if self.delta.abs() > 1.0 {
                    self.delta / 100.0
                } else {
                    self.delta
                }
            }
        };

        let pose_values = state.poses.get_mut(current_pose);

        let vec = match edit_target {
            EditTarget::HandOffset => &mut pose_values.hand_offset,
            EditTarget::ElbowHint => &mut pose_values.elbow_hint,
            EditTarget::HandRotation => &mut pose_values.rotation_degrees,
        };

        match axis {
            0 => vec.x += actual_delta,
            1 => vec.y += actual_delta,
            _ => vec.z += actual_delta,
        }
    }
}

struct PrintAllPosesAction;
impl UiAction for PrintAllPosesAction {
    fn execute(&self, world: &mut World) {
        let state = world.resource::<PoseDebugState>();

        info!("");
        info!("════════════════════════════════════════════════════════════════");
        info!("🎨 ALL POSES (copy to components.rs):");
        info!("════════════════════════════════════════════════════════════════");

        for &pose_type in DebugPose::ALL {
            let pose = state.poses.get(pose_type);
            info!("");
            info!("pub fn {}() -> Self {{", pose_type.fn_name());
            info!("    Self {{");
            info!(
                "        hand_offset: Vec3::new({:.2}, {:.2}, {:.2}),",
                pose.hand_offset.x, pose.hand_offset.y, pose.hand_offset.z
            );
            info!(
                "        elbow_hint: Vec3::new({:.2}, {:.2}, {:.2}),",
                pose.elbow_hint.x, pose.elbow_hint.y, pose.elbow_hint.z
            );
            info!(
                "        hand_rotation: Self::rot({:.1}, {:.1}, {:.1}),",
                pose.rotation_degrees.x, pose.rotation_degrees.y, pose.rotation_degrees.z
            );
            info!("    }}");
            info!("}}");
        }

        info!("");
        info!("════════════════════════════════════════════════════════════════");
    }
}

struct ClosePoseDebugAction;
impl UiAction for ClosePoseDebugAction {
    fn execute(&self, world: &mut World) {
        let mut state = world.resource_mut::<PoseDebugState>();
        state.enabled = false;
        info!("🎨 Pose Debug: OFF");
    }
}

// ═══════════════════════════════════════════════════════════════════
// SYNC SYSTEM
// ═══════════════════════════════════════════════════════════════════

pub fn sync_pose_debug_ui(
    state: Res<PoseDebugState>,
    mut pose_name: Query<
        &mut Text,
        (
            With<PoseNameText>,
            Without<EditTargetText>,
            Without<AxisText>,
            Without<CurrentValueText>,
            Without<PoseValuesText>,
        ),
    >,
    mut edit_target: Query<
        &mut Text,
        (
            With<EditTargetText>,
            Without<PoseNameText>,
            Without<AxisText>,
            Without<CurrentValueText>,
            Without<PoseValuesText>,
        ),
    >,
    mut axis_text: Query<
        &mut Text,
        (
            With<AxisText>,
            Without<PoseNameText>,
            Without<EditTargetText>,
            Without<CurrentValueText>,
            Without<PoseValuesText>,
        ),
    >,
    mut current_value: Query<
        &mut Text,
        (
            With<CurrentValueText>,
            Without<PoseNameText>,
            Without<EditTargetText>,
            Without<AxisText>,
            Without<PoseValuesText>,
        ),
    >,
    mut pose_values: Query<
        &mut Text,
        (
            With<PoseValuesText>,
            Without<PoseNameText>,
            Without<EditTargetText>,
            Without<AxisText>,
            Without<CurrentValueText>,
        ),
    >,
) {
    if !state.enabled {
        return;
    }

    // Pose name
    for mut text in &mut pose_name {
        **text = state.current_pose.name().to_string();
    }

    // Edit target
    for mut text in &mut edit_target {
        **text = match state.edit_target {
            EditTarget::HandOffset => "hand_offset",
            EditTarget::ElbowHint => "elbow_hint",
            EditTarget::HandRotation => "rotation",
        }
        .to_string();
    }

    // Axis
    for mut text in &mut axis_text {
        **text = match (state.edit_target, state.axis) {
            (EditTarget::HandRotation, 0) => "X (pitch)",
            (EditTarget::HandRotation, 1) => "Y (yaw)",
            (EditTarget::HandRotation, 2) => "Z (roll)",
            (_, 0) => "X",
            (_, 1) => "Y",
            _ => "Z",
        }
        .to_string();
    }

    // Current value
    let pose_data = state.poses.get(state.current_pose);
    let vec = match state.edit_target {
        EditTarget::HandOffset => &pose_data.hand_offset,
        EditTarget::ElbowHint => &pose_data.elbow_hint,
        EditTarget::HandRotation => &pose_data.rotation_degrees,
    };
    let value = match state.axis {
        0 => vec.x,
        1 => vec.y,
        _ => vec.z,
    };

    for mut text in &mut current_value {
        if matches!(state.edit_target, EditTarget::HandRotation) {
            **text = format!("{:.1}°", value);
        } else {
            **text = format!("{:.2}", value);
        }
    }

    // All pose values
    for mut text in &mut pose_values {
        **text = format!(
            "hand_offset: ({:.2}, {:.2}, {:.2})\nelbow_hint: ({:.2}, {:.2}, {:.2})\nrotation: ({:.1}, {:.1}, {:.1})",
            pose_data.hand_offset.x,
            pose_data.hand_offset.y,
            pose_data.hand_offset.z,
            pose_data.elbow_hint.x,
            pose_data.elbow_hint.y,
            pose_data.elbow_hint.z,
            pose_data.rotation_degrees.x,
            pose_data.rotation_degrees.y,
            pose_data.rotation_degrees.z,
        );
    }
}
