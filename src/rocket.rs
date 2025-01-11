use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
pub struct Rocket {
    pub speed: f32,
    pub max_speed: f32,
    pub velocity: Vec2,
    pub rotation_speed: f32,
    pub radius_collision: f32,
    pub controls: RocketControls,
    pub spawn_key: KeyCode,
    pub cooldown: Duration,
    pub last_shot_time: Duration,
}

pub struct RocketControls {
    pub rotate_left: KeyCode,
    pub rotate_right: KeyCode,
    pub accelerate: KeyCode,
}

const W_CLIP: f32 = 510.;
const H_CLIP: f32 = 380.;

const SPAWN_X: f32 = 400.;
const SPAWN_Y: f32 = 200.;

pub fn add_rockets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let rocket_image = asset_server.load("Rocket1-v2.png");
    let rocket_image_b = asset_server.load("Rocket1-v3.png");

    let pos = Vec2::new(-SPAWN_X, SPAWN_Y);
    let pos_b = Vec2::new(SPAWN_X, -SPAWN_Y);
    let direction = Vec2::ZERO - pos;
    let direction_b = Vec2::ZERO - pos_b;
    let angle = direction.y.atan2(direction.x) - std::f32::consts::FRAC_PI_2;
    let angle_b = direction_b.y.atan2(direction_b.x) - std::f32::consts::FRAC_PI_2;

    let rocket_a_pos = Transform {
        translation: Vec3::new(-SPAWN_X, SPAWN_Y, 1.),
        scale: Vec3::splat(0.12),
        rotation: Quat::from_rotation_z(angle),
        ..Default::default()
    };
    let rocket_b_pos = Transform {
        translation: Vec3::new(SPAWN_X, -SPAWN_Y, 1.),
        scale: Vec3::splat(0.144),
        rotation: Quat::from_rotation_z(angle_b),
        ..Default::default()
    };
    commands.spawn((
        SpriteBundle {
            texture: rocket_image.clone(),
            transform: rocket_a_pos,
            ..default()
        },
        Rocket {
            speed: 0.0,
            max_speed: 150.0,
            velocity: Vec2::ZERO,
            radius_collision: 50.,
            rotation_speed: f32::to_radians(0.0),
            controls: RocketControls {
                rotate_left: KeyCode::KeyA,
                rotate_right: KeyCode::KeyD,
                accelerate: KeyCode::KeyS,
            },
            spawn_key: KeyCode::KeyW,
            cooldown: Duration::from_millis(500),
            last_shot_time: Duration::ZERO,
        },
    ));
    commands.spawn((
        SpriteBundle {
            texture: rocket_image_b,
            transform: rocket_b_pos,
            ..default()
        },
        Rocket {
            speed: 0.0,
            max_speed: 150.0,
            radius_collision: 50.,
            velocity: Vec2::ZERO,
            rotation_speed: f32::to_radians(0.0),
            controls: RocketControls {
                rotate_left: KeyCode::KeyJ,
                rotate_right: KeyCode::KeyL,
                accelerate: KeyCode::KeyK,
            },
            spawn_key: KeyCode::KeyI,
            cooldown: Duration::from_millis(300),
            last_shot_time: Duration::ZERO
        },
    ));
}

pub fn clip_rockets(mut query: Query<&mut Transform, With<Rocket>>) {
    let mut pos: Vec<Mut<Transform>> = query.iter_mut().collect();
    for i in 0..pos.len() {
        let transform = &mut pos[i];
        if transform.translation[0] < -W_CLIP || transform.translation[0] > W_CLIP {
            transform.translation = Vec3::new(
                -transform.translation[0],
                transform.translation[1],
                transform.translation[2],
            );
        }
        if transform.translation[1] < -H_CLIP || transform.translation[1] > H_CLIP {
            transform.translation = Vec3::new(
                transform.translation[0],
                -transform.translation[1],
                transform.translation[2],
            );
        }
    }
}
