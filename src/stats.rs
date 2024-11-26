use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy::render::view::visibility;
use bevy_rapier3d::prelude::*;
use crate::player::Player;

pub struct StatsPlugin;

impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, init_stats)
            .add_systems(Update, (update_stats, show_stats));
    }
}

#[derive(Component)]
pub struct StatsText;

fn init_stats(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let text = TextBundle {
        style: Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        text: Text::from_section(
            "STATS",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 12.0,
                color: Color::WHITE
            },
        ),
        ..default()
    };

    commands.spawn(text)
        .insert(StatsText)
        .insert(Visibility::Hidden);
}

fn update_stats(
    mut query: Query<&mut Text, With<StatsText>>,
    player_query: Query<(&Velocity, &Transform), With<Player>>,
    diagnostics: Res<DiagnosticsStore>,
) {
    if let Ok(mut text) = query.get_single_mut() {
        if let Ok((velocity, transform)) = player_query.get_single() {
            let fps = diagnostics
                .get(FrameTimeDiagnosticsPlugin::FPS)
                .and_then(|d| d.value())
                .unwrap_or(0.0);

            // velocity threshold
            const EPSILON: f32 = 0.0001;

            // prevents velocity flickering
            let linvel_x = if velocity.linvel.x.abs() < EPSILON {
                0.0
            } else {
                velocity.linvel.x
            };

            let linvel_y = if velocity.linvel.y.abs() < EPSILON {
                0.0
            } else {
                velocity.linvel.y
            };

            let linvel_z = if velocity.linvel.z.abs() < EPSILON {
                0.0
            } else {
                velocity.linvel.z
            };

            text.sections[0].value = format!(
                "FPS: {:.2}\nPosition: x={:.2}, y={:.2}, z={:.2}\nVelocity: x={:.2}, y={:.2}, z={:.2}",
                fps,
                transform.translation.x,
                transform.translation.y,
                transform.translation.z,
                linvel_x,
                linvel_y,
                linvel_z,
            );
        }
    }
}

fn show_stats(
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut Visibility, With<StatsText>>,
) {
    if keys.just_pressed(KeyCode::F1) {
        for mut visibility in query.iter_mut() {
            *visibility = match *visibility {
                Visibility::Visible => Visibility::Hidden,
                Visibility::Hidden => Visibility::Visible,
                _ => Visibility::Hidden
            };
        }
    }
}