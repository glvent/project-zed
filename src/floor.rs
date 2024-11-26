use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct FloorPlugin;

impl Plugin for FloorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_floor);
    }
}

fn spawn_floor(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let floor = SceneBundle {
        scene: asset_server.load("models/western_terrain.glb#Scene0"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    };

    commands.spawn(floor).insert(Collider::cuboid(75.0, 0.1, 75.0));
}