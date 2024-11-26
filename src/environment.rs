use bevy::prelude::*;

pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_light);
    }
}

fn spawn_light(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 1000.0, // Adjust intensity
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(-2.0, 4.0, -2.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    commands.insert_resource(AmbientLight {
        color: Color::rgb(0.5, 0.5, 0.5),
        brightness: 1.0,
    })
}