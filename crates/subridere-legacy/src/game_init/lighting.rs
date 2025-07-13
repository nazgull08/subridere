use bevy::prelude::*;

pub fn setup_ambient_light(mut _commands: Commands) {
    /*
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.25, 0.25, 0.25), // тускло-серый фон
        brightness: 1.00,                     // 0.0 – тьма, 1.0 – ярко
        affects_lightmapped_meshes: true,
    });
    */
    // Заливаем фон кадра чёрным, чтобы «не светился» скайбокс-по-умолчанию
    //commands.insert_resource(ClearColor(Color::BLACK));
}

pub fn spawn_lighting(mut commands: Commands) {
    println!("spawn sun");
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 10000000.0,
            range: 50.0,
            ..default()
        },
        Transform::from_xyz(10.0, 20.0, 10.0),
        Name::new("MainLight"),
    ));
}
