use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy::core_pipeline::bloom::BloomSettings;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (init_player, setup_cursor))
            .add_systems(Update, (handle_movement, handle_mouse_look));
    }
}

#[derive(Component)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Component)]
pub struct FirstPersonCamera {
    pub pitch: f32,
    pub yaw: f32,
}

fn init_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule {
                radius: 0.5,
                depth: 1.0,
                ..default()
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.2, 0.7, 0.2),
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        }
    ))
        .insert(Player { x: 0.0, y: 0.0, z: 0.0 })
        .insert(FirstPersonCamera { pitch: 0.0, yaw: 0.0 })
        .insert(RigidBody::Dynamic)
        .insert(Velocity::zero())
        .insert(Collider::capsule_y(0.5, 1.0))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Damping {
            linear_damping: 0.5,
            angular_damping: 0.5,
        })
        .insert(GravityScale {
        0: 2.0,
        })
        .with_children(|parent| {
            parent.spawn(Camera3dBundle {
                camera: Camera {
                    hdr: true,
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 1.5, 0.0),
                ..default()
            }).insert(BloomSettings {
                intensity: 0.8,
                ..default()
            });
        });
}

fn handle_movement(
    keys: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Velocity, With<Player>>,
    camera_query: Query<&Transform, With<FirstPersonCamera>>,
) {
    let mut velocity = player_query.single_mut();
    let camera_transform = camera_query.single();
    const BASE_SPEED: f32 = 10.0;
    const SPRINT_MULTIPLIER: f32 = 1.5;

    let mut direction = Vec3::ZERO;
    let forward = Vec3::new(camera_transform.forward().x, 0.0, camera_transform.forward().z).normalize_or_zero();
    let right = Vec3::new(camera_transform.right().x, 0.0, camera_transform.right().z).normalize_or_zero();

    let can_jump = velocity.linvel.y.abs() < 0.0001;

    if keys.pressed(KeyCode::W) {
        direction += forward;
    }
    if keys.pressed(KeyCode::S) {
        direction -= forward;
    }
    if keys.pressed(KeyCode::A) {
        direction -= right;
    }
    if keys.pressed(KeyCode::D) {
        direction += right;
    }

     if direction.length_squared() > 0.0 {
         if keys.pressed(KeyCode::ShiftLeft) && keys.pressed(KeyCode::W) && can_jump {
             direction *= SPRINT_MULTIPLIER
         } else {
             direction = direction.normalize();
         }
     }

    let current_vertical_velocity = velocity.linvel.y;
    velocity.linvel = direction * BASE_SPEED;
    velocity.linvel.y = current_vertical_velocity;

    // 0.0001 is y-velo before u can jump again
    if keys.just_pressed(KeyCode::Space) && can_jump {
        velocity.linvel.y = BASE_SPEED;
    }
}


fn setup_cursor(mut windows: Query<&mut Window>) {
    let mut window = windows.get_single_mut().unwrap();
    window.cursor.visible = false;
    window.cursor.grab_mode = bevy::window::CursorGrabMode::Locked;
}
fn handle_mouse_look(
    mut motion_events: EventReader<MouseMotion>,
    mut query: ParamSet<(
        Query<&mut Transform, With<Player>>,
        Query<(&mut Transform, &mut FirstPersonCamera)>,
    )>,
) {
    let mut delta = Vec2::ZERO;

    for event in motion_events.iter() {
        delta += event.delta;
    }

    if delta == Vec2::ZERO {
        return;
    }

    let sensitivity = 0.005;

    // handle yaw rotation (horizontal movement) for the player
    if let Ok(mut player_transform) = query.p0().get_single_mut() {
        player_transform.rotate_y(-delta.x * sensitivity);
    }

    // handle pitch rotation (vertical movement) for the camera
    if let Ok((mut camera_transform, mut camera)) = query.p1().get_single_mut() {
        let pitch_delta = -delta.y * sensitivity;
        let new_pitch = (camera.pitch + pitch_delta)
            .clamp(-std::f32::consts::FRAC_PI_4, std::f32::consts::FRAC_PI_4);

        let pitch_change = new_pitch - camera.pitch;
        camera.pitch = new_pitch;
        camera_transform.rotate_local_x(pitch_change);
    }
}

