// crates/subridere-core/src/player/body/visible_body.rs
//
// Видимое тело игрока для first-person (чисто визуальное)

use bevy::prelude::*;

/// Маркер видимого тела
#[derive(Component)]
pub struct VisibleBody;

/// Маркер торса
#[derive(Component)]
pub struct Torso;

/// Маркер ног
#[derive(Component)]
pub struct Legs;

/// Конфиг видимого тела
#[derive(Resource)]
pub struct VisibleBodyConfig {
    /// Смещение торса относительно камеры
    pub torso_offset: Vec3,
    /// Размер торса
    pub torso_size: Vec3,
    /// Смещение ног относительно торса
    pub legs_offset: Vec3,
    /// Размер ног (обе вместе)
    pub legs_size: Vec3,
}

impl Default for VisibleBodyConfig {
    fn default() -> Self {
        Self {
            // Торс: ниже уровня глаз, немного вперёд
            // EYE_HEIGHT = 0.7, значит торс примерно на 0.0..0.3
            torso_offset: Vec3::new(0.00, 0.15, 0.25),
            torso_size: Vec3::new(0.40, 0.50, 0.25),

            legs_offset: Vec3::new(0.00, -0.55, 0.00),
            legs_size: Vec3::new(0.38, 0.60, 0.22),
        }
    }
}

/// Спавнит видимое тело как child камеры
pub fn spawn_visible_body(
    commands: &mut Commands,
    camera_entity: Entity,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    config: &VisibleBodyConfig,
) {
    // Материал тела (одежда — тёмная)
    let body_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.15, 0.15, 0.18), // Тёмно-серый
        metallic: 0.0,
        perceptual_roughness: 0.9,
        ..default()
    });

    // Материал ног (штаны — чуть светлее)
    let legs_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.12, 0.10, 0.08), // Тёмно-коричневый
        metallic: 0.0,
        perceptual_roughness: 0.95,
        ..default()
    });

    // === Корневая сущность ===
    let body_root = commands
        .spawn((
            VisibleBody,
            Transform::default(),
            GlobalTransform::default(),
            Visibility::Inherited,
            Name::new("VisibleBody"),
        ))
        .id();

    commands.entity(camera_entity).add_child(body_root);

    // === Торс ===
    let torso_mesh = meshes.add(Cuboid::new(
        config.torso_size.x,
        config.torso_size.y,
        config.torso_size.z,
    ));

    let torso_entity = commands
        .spawn((
            Torso,
            Mesh3d(torso_mesh),
            MeshMaterial3d(body_material),
            Transform::from_translation(config.torso_offset),
            GlobalTransform::default(),
            Visibility::Inherited,
            Name::new("Torso"),
        ))
        .id();

    commands.entity(body_root).add_child(torso_entity);

    // === Ноги ===
    let legs_mesh = meshes.add(Cuboid::new(
        config.legs_size.x,
        config.legs_size.y,
        config.legs_size.z,
    ));

    let legs_entity = commands
        .spawn((
            Legs,
            Mesh3d(legs_mesh),
            MeshMaterial3d(legs_material),
            Transform::from_translation(config.torso_offset + config.legs_offset),
            GlobalTransform::default(),
            Visibility::Inherited,
            Name::new("Legs"),
        ))
        .id();

    commands.entity(body_root).add_child(legs_entity);

    info!("✅ Visible body spawned (torso + legs)");
}

/// Debug state для настройки позиции тела
#[derive(Resource)]
pub struct BodyDebugState {
    pub current_axis: usize,
    pub step: f32,
    pub enabled: bool,
}

impl Default for BodyDebugState {
    fn default() -> Self {
        Self {
            current_axis: 1, // Y
            step: 0.05,
            enabled: true,
        }
    }
}

/// Debug система — F7/F8 для тела
pub fn body_debug_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut debug_state: ResMut<BodyDebugState>,
    mut config: ResMut<VisibleBodyConfig>,
    mut torso_query: Query<&mut Transform, (With<Torso>, Without<Legs>)>,
    mut legs_query: Query<&mut Transform, (With<Legs>, Without<Torso>)>,
) {
    if !debug_state.enabled {
        return;
    }

    // F7 — переключить ось
    if keyboard.just_pressed(KeyCode::F7) {
        debug_state.current_axis = (debug_state.current_axis + 1) % 3;
        let axis_name = match debug_state.current_axis {
            0 => "X",
            1 => "Y",
            2 => "Z",
            _ => "?",
        };
        info!("🎮 BODY DEBUG: axis = {}", axis_name);
    }

    // F8 — вывести значения
    if keyboard.just_pressed(KeyCode::F8) {
        info!("════════════════════════════════════════");
        info!("🦴 BODY CONFIG:");
        info!(
            "   torso_offset: Vec3::new({:.2}, {:.2}, {:.2}),",
            config.torso_offset.x, config.torso_offset.y, config.torso_offset.z
        );
        info!(
            "   legs_offset: Vec3::new({:.2}, {:.2}, {:.2}),",
            config.legs_offset.x, config.legs_offset.y, config.legs_offset.z
        );
        info!("════════════════════════════════════════");
    }

    // PageUp/PageDown — изменить torso_offset
    let mut delta = 0.0;
    if keyboard.just_pressed(KeyCode::PageUp) {
        delta = debug_state.step;
    }
    if keyboard.just_pressed(KeyCode::PageDown) {
        delta = -debug_state.step;
    }

    if delta != 0.0 {
        match debug_state.current_axis {
            0 => config.torso_offset.x += delta,
            1 => config.torso_offset.y += delta,
            2 => config.torso_offset.z += delta,
            _ => {}
        }

        // Обновить transform
        for mut transform in &mut torso_query {
            transform.translation = config.torso_offset;
        }
        for mut transform in &mut legs_query {
            transform.translation = config.torso_offset + config.legs_offset;
        }

        info!(
            "🎮 torso_offset.{} = {:.2}",
            match debug_state.current_axis {
                0 => "X",
                1 => "Y",
                2 => "Z",
                _ => "?",
            },
            match debug_state.current_axis {
                0 => config.torso_offset.x,
                1 => config.torso_offset.y,
                2 => config.torso_offset.z,
                _ => 0.0,
            }
        );
    }
}
