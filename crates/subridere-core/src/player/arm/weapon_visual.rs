// player/arm/weapon_visual.rs
//
// Система визуализации экипированного оружия в руке

use bevy::prelude::*;

use crate::fighting::arm_to_slot;
use crate::inventory::Equipment;
use crate::items::{ItemRegistry, ItemVisual, VisualPart, VisualShape};
use crate::player::component::Player;

use super::components::{ArmSide, WeaponSocket};

// ═══════════════════════════════════════════════════════════════════
// COMPONENTS
// ═══════════════════════════════════════════════════════════════════

/// Маркер для визуала экипированного оружия
#[derive(Component)]
pub struct EquippedWeaponVisual {
    pub side: ArmSide,
}

// ═══════════════════════════════════════════════════════════════════
// DEBUG SYSTEM
// ═══════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponDebugAxis {
    TranslationX,
    TranslationY,
    TranslationZ,
    RotationX,
    RotationY,
    RotationZ,
}

impl WeaponDebugAxis {
    fn next(self) -> Self {
        match self {
            Self::TranslationX => Self::TranslationY,
            Self::TranslationY => Self::TranslationZ,
            Self::TranslationZ => Self::RotationX,
            Self::RotationX => Self::RotationY,
            Self::RotationY => Self::RotationZ,
            Self::RotationZ => Self::TranslationX,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::TranslationX => "Translation X",
            Self::TranslationY => "Translation Y",
            Self::TranslationZ => "Translation Z",
            Self::RotationX => "Rotation X (pitch)",
            Self::RotationY => "Rotation Y (yaw)",
            Self::RotationZ => "Rotation Z (roll)",
        }
    }
}

#[derive(Resource)]
pub struct WeaponDebugState {
    pub enabled: bool,
    pub axis: WeaponDebugAxis,
    pub translation: Vec3,
    pub rotation_degrees: Vec3,
}

impl Default for WeaponDebugState {
    fn default() -> Self {
        Self {
            enabled: false,
            axis: WeaponDebugAxis::TranslationX,
            // Найденные через дебаг правильные значения
            translation: Vec3::new(0.04, -0.08, -0.14),
            rotation_degrees: Vec3::new(-150.0, 80.0, 5.0),
        }
    }
}

impl WeaponDebugState {
    pub fn to_transform(&self) -> Transform {
        let rotation = Quat::from_euler(
            EulerRot::XYZ,
            self.rotation_degrees.x.to_radians(),
            self.rotation_degrees.y.to_radians(),
            self.rotation_degrees.z.to_radians(),
        );

        Transform {
            translation: self.translation,
            rotation,
            scale: Vec3::ONE,
        }
    }
}

/// Система дебага — F7 toggle, F8 switch axis, стрелки adjust
pub fn weapon_debug_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut weapon_debug: ResMut<WeaponDebugState>,
) {
    // F7 — toggle debug
    if keyboard.just_pressed(KeyCode::F7) {
        weapon_debug.enabled = !weapon_debug.enabled;
        if weapon_debug.enabled {
            info!("🗡️ WEAPON DEBUG: ON");
            info!("   F8 = switch axis, ↑/↓ = adjust, F9 = print values");
            info!("   Current axis: {}", weapon_debug.axis.name());
        } else {
            info!("🗡️ WEAPON DEBUG: OFF");
        }
    }

    if !weapon_debug.enabled {
        return;
    }

    // F8 — switch axis
    if keyboard.just_pressed(KeyCode::F8) {
        weapon_debug.axis = weapon_debug.axis.next();
        info!("🗡️ Axis: {}", weapon_debug.axis.name());
    }

    // F9 — print current values
    if keyboard.just_pressed(KeyCode::F9) {
        info!("════════════════════════════════════════════════════");
        info!("🗡️ WEAPON GRIP VALUES:");
        info!(
            "   translation: Vec3::new({:.2}, {:.2}, {:.2})",
            weapon_debug.translation.x, weapon_debug.translation.y, weapon_debug.translation.z
        );
        info!(
            "   rotation_deg: Vec3::new({:.1}, {:.1}, {:.1})",
            weapon_debug.rotation_degrees.x,
            weapon_debug.rotation_degrees.y,
            weapon_debug.rotation_degrees.z
        );
        info!("════════════════════════════════════════════════════");
    }

    // Arrows — adjust
    let step_translation = 0.02;
    let step_rotation = 5.0;

    let delta = if keyboard.just_pressed(KeyCode::ArrowUp) {
        1.0
    } else if keyboard.just_pressed(KeyCode::ArrowDown) {
        -1.0
    } else {
        return;
    };

    match weapon_debug.axis {
        WeaponDebugAxis::TranslationX => weapon_debug.translation.x += delta * step_translation,
        WeaponDebugAxis::TranslationY => weapon_debug.translation.y += delta * step_translation,
        WeaponDebugAxis::TranslationZ => weapon_debug.translation.z += delta * step_translation,
        WeaponDebugAxis::RotationX => weapon_debug.rotation_degrees.x += delta * step_rotation,
        WeaponDebugAxis::RotationY => weapon_debug.rotation_degrees.y += delta * step_rotation,
        WeaponDebugAxis::RotationZ => weapon_debug.rotation_degrees.z += delta * step_rotation,
    }

    let current_value = match weapon_debug.axis {
        WeaponDebugAxis::TranslationX => weapon_debug.translation.x,
        WeaponDebugAxis::TranslationY => weapon_debug.translation.y,
        WeaponDebugAxis::TranslationZ => weapon_debug.translation.z,
        WeaponDebugAxis::RotationX => weapon_debug.rotation_degrees.x,
        WeaponDebugAxis::RotationY => weapon_debug.rotation_degrees.y,
        WeaponDebugAxis::RotationZ => weapon_debug.rotation_degrees.z,
    };
    info!("🗡️ {} = {:.2}", weapon_debug.axis.name(), current_value);
}

/// Применяет дебаг-трансформ к оружию в реальном времени
pub fn apply_weapon_debug_transform(
    weapon_debug: Res<WeaponDebugState>,
    mut visual_query: Query<&mut Transform, With<EquippedWeaponVisual>>,
) {
    if !weapon_debug.enabled {
        return;
    }

    let target_transform = weapon_debug.to_transform();

    for mut transform in &mut visual_query {
        *transform = target_transform;
    }
}

// ═══════════════════════════════════════════════════════════════════
// SYNC SYSTEM
// ═══════════════════════════════════════════════════════════════════

/// Система синхронизации визуала оружия с экипировкой
pub fn sync_equipped_weapon_visual(
    mut commands: Commands,
    player_query: Query<&Equipment, (With<Player>, Changed<Equipment>)>,
    weapon_socket_query: Query<(Entity, &WeaponSocket, Option<&Children>)>,
    visual_query: Query<Entity, With<EquippedWeaponVisual>>,
    registry: Res<ItemRegistry>,
    weapon_debug: Res<WeaponDebugState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Ok(equipment) = player_query.single() else {
        return;
    };

    for (socket_entity, socket, children) in &weapon_socket_query {
        let slot = arm_to_slot(socket.side);
        let item_id = equipment.get(slot);

        // Удаляем старый визуал
        if let Some(children) = children {
            for child in children.iter() {
                if visual_query.get(child).is_ok() {
                    commands.entity(child).despawn_recursive();
                }
            }
        }

        // Спавним новый визуал если есть оружие
        let Some(id) = item_id else {
            continue;
        };

        let def = registry.get(id);
        let ItemVisual::Primitive { parts } = &def.visual else {
            continue;
        };

        // Используем дебаг-трансформ если включён, иначе дефолтный
        let visual_transform = if weapon_debug.enabled {
            weapon_debug.to_transform()
        } else {
            weapon_grip_transform(socket.side)
        };

        let visual_entity = commands
            .spawn((
                EquippedWeaponVisual { side: socket.side },
                visual_transform,
                GlobalTransform::default(),
                Visibility::Inherited,
                Name::new(format!("EquippedWeapon_{:?}", socket.side)),
            ))
            .id();

        // Спавним части визуала напрямую
        spawn_weapon_parts(
            &mut commands,
            visual_entity,
            parts,
            &mut meshes,
            &mut materials,
        );

        // Присоединяем к сокету
        commands.entity(socket_entity).add_child(visual_entity);

        let side_name = match socket.side {
            ArmSide::Right => "RIGHT",
            ArmSide::Left => "LEFT",
        };
        info!("🗡️ {} hand: equipped '{}'", side_name, def.name);
    }
}

// ═══════════════════════════════════════════════════════════════════
// HELPERS
// ═══════════════════════════════════════════════════════════════════

/// Спавнит части визуала оружия
fn spawn_weapon_parts(
    commands: &mut Commands,
    parent: Entity,
    parts: &[VisualPart],
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    for part in parts {
        let mesh = create_mesh_for_shape(part.shape, part.size_vec3(), meshes);
        let material = materials.add(StandardMaterial {
            base_color: part.bevy_color(),
            ..default()
        });

        let child = commands
            .spawn((
                Mesh3d(mesh),
                MeshMaterial3d(material),
                Transform::from_translation(part.offset_vec3()),
            ))
            .id();

        commands.entity(parent).add_child(child);
    }
}

fn create_mesh_for_shape(
    shape: VisualShape,
    size: Vec3,
    meshes: &mut Assets<Mesh>,
) -> Handle<Mesh> {
    match shape {
        VisualShape::Cube => meshes.add(Cuboid::new(size.x, size.y, size.z)),
        VisualShape::Sphere => meshes.add(Sphere::new(size.x)),
        VisualShape::Cylinder => meshes.add(Cylinder::new(size.x, size.y)),
        VisualShape::Capsule => meshes.add(Capsule3d::new(size.x, size.y)),
    }
}

/// Трансформ для правильного хвата оружия
/// Найдено через дебаг (F7/F9)
fn weapon_grip_transform(_side: ArmSide) -> Transform {
    let rotation = Quat::from_euler(
        EulerRot::XYZ,
        (-150.0_f32).to_radians(),
        (80.0_f32).to_radians(),
        (5.0_f32).to_radians(),
    );

    Transform {
        translation: Vec3::new(0.04, -0.08, -0.14),
        rotation,
        scale: Vec3::ONE,
    }
}
