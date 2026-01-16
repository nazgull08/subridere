use bevy::ecs::event::Event;
use bevy::prelude::*;

use crate::core::components::GameEntity; // ← ДОБАВИТЬ

#[derive(Event)]
pub struct HitFlashEvent;

#[derive(Component)]
pub struct HitOverlay {
    pub timer: Timer,
    pub initial_alpha: f32,
}

pub fn spawn_hit_overlay(
    mut commands: Commands,
    mut evr: EventReader<HitFlashEvent>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/dogica.ttf");

    for _ in evr.read() {
        let initial_alpha = 0.6;

        commands
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.0),
                    left: Val::Px(0.0),
                    width: Val::Vw(100.0),
                    height: Val::Vh(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgba(1.0, 0.2, 0.2, initial_alpha)),
                Name::new("HitOverlay"),
                HitOverlay {
                    timer: Timer::from_seconds(1.0, TimerMode::Once),
                    initial_alpha,
                },
                GameEntity, // ← ДОБАВИТЬ - удалится при cleanup
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("That was painful!"),
                    TextFont {
                        font: font.clone(),
                        font_size: 48.0,
                        ..default()
                    },
                    TextColor(Color::srgba(1.0, 1.0, 1.0, 1.0)),
                    Node {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ));
            });
    }
}

pub fn update_hit_overlay(
    time: Res<Time>,
    mut commands: Commands,
    mut q: Query<(Entity, &mut HitOverlay, &mut BackgroundColor)>,
) {
    for (entity, mut overlay, mut bg_color) in &mut q {
        overlay.timer.tick(time.delta());

        let progress = 1.0 - overlay.timer.fraction();
        let current_alpha = overlay.initial_alpha * progress;
        bg_color.0.set_alpha(current_alpha);

        if overlay.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}
