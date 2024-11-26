mod player;
mod environment;
mod floor;
mod hud;
mod stats;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::floor::FloorPlugin;
use crate::environment::LightPlugin;
use crate::player::PlayerPlugin;
use crate::stats::StatsPlugin;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins((
            DefaultPlugins,
            FrameTimeDiagnosticsPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            StatsPlugin,
            PlayerPlugin,
            LightPlugin,
            FloorPlugin
        ))
        .run();
}
