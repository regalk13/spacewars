use bevy::{
    prelude::*,
    render::render_resource::*,
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
    window::WindowMode,
};

mod post_process;
mod rocket;
mod bullet;
use rocket::{add_rockets, clip_rockets, Rocket};
use bullet::{check_bullet_coll, spawn_bullet};

#[derive(Asset, TypePath, AsBindGroup, Clone)]
pub struct MovingPatternMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[uniform(1)]
    time: f32,
}

impl Material2d for MovingPatternMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/stars_material.wgsl".into()
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "SpaceWars".into(),
                    name: Some("bevy.app".into()),
                    resolution: (980., 735.).into(),
                    mode: WindowMode::Windowed,
                    resizable: false,
                    ..default()
                }),
                ..default()
            }),
            post_process::PostProcessPlugin,
            Material2dPlugin::<MovingPatternMaterial>::default(),
        ))
        .add_systems(
            Startup,
            (setup, add_background, add_sun, add_rockets).chain(),
        )
        .add_systems(
            Update,
            (
                check_bullet_coll,
                spawn_bullet,
                update_rocket_status,
                clip_rockets,
                gravitational_pull,
                post_process::rotate,
                post_process::update_settings,
            )
                .chain(),
        )
        .run();
}

fn check_collision(rocket1: &Transform, rocket2: &Transform, radius_collison: f32) -> bool {
    let distance = rocket1
        .translation
        .truncate()
        .distance(rocket2.translation.truncate());
    println!("{}", distance);
    distance < radius_collison
}

fn check_sun_collision(rocket: &Transform, radius_collision: f32) -> bool {
    let distance = rocket
        .translation
        .truncate()
        .distance(Vec2::new(0.0, 0.0));
    distance < radius_collision
}

fn gravitational_pull(mut rocket_query: Query<(&mut Rocket, &mut Transform)>, time: Res<Time>) {
    let sun_position = Vec2::ZERO;
    const G_FORCE: f64 = 125000000.0;

    for (mut rocket, mut transform) in rocket_query.iter_mut() {
        let rocket_position = Vec2::new(transform.translation.x, transform.translation.y);

        let direction = sun_position - rocket_position;
        let distance = direction.length();

        if distance < 65.0 {
            continue;
        }

        let force = G_FORCE / (distance * distance) as f64;

        let acceleration = direction.normalize() * force as f32;

        rocket.velocity += acceleration * time.delta_seconds();

        if rocket.velocity.length() > rocket.max_speed {
            rocket.velocity = rocket.velocity.normalize() * rocket.max_speed;
        }
        transform.translation.x += rocket.velocity.x * time.delta_seconds();
        transform.translation.y += rocket.velocity.y * time.delta_seconds();
    }
}

fn add_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<MovingPatternMaterial>>,
    window: Query<&Window>,
) {
    let window = window.single();
    let resolution = Vec2::new(window.width(), window.height());
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(Mesh::from(Rectangle::from_size(resolution)))
            .into(),
        material: materials.add(MovingPatternMaterial {
            color: LinearRgba::WHITE,
            time: 0.0,
        }),
        ..default()
    });
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera { ..default() },
            ..default()
        },
        post_process::PostProcessSettings {
            intensity: 0.02,
            ..default()
        },
    ));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 1_000.,
            ..default()
        },
        ..default()
    });
}

#[derive(Component)]
struct Sun {}

fn add_sun(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(Circle::new(50.0))).into(),
            material: materials.add(ColorMaterial::from(Color::srgb(7.0, 7.0, 0.0))),
            transform: Transform::from_xyz(0.0, 0.0, 3.0),
            ..default()
        },
        Sun {},
    ));
}

fn handle_rocket_movement(
    time: &Res<Time>,
    keys: &Res<ButtonInput<KeyCode>>,
    rocket: &mut Rocket,
    transform: &mut Transform,
) {
    if keys.pressed(rocket.controls.accelerate) {
        if rocket.speed < rocket.max_speed {
            rocket.speed += 50.0 * time.delta_seconds();
        }
    } else {
        if rocket.speed > 0.0 {
            rocket.speed -= 50.0 * time.delta_seconds();
        }
    }

    let mut rotation_input = 0.0;

    if keys.pressed(rocket.controls.rotate_left) {
        rotation_input += 4.0;
    }

    if keys.pressed(rocket.controls.rotate_right) {
        rotation_input -= 4.0;
    }

    let max_rotation_speed = f32::to_radians(70.0);
    let rotation_acceleration = f32::to_radians(50.0 * time.delta_seconds());
    rocket.rotation_speed += rotation_input * rotation_acceleration;
    rocket.rotation_speed = rocket
        .rotation_speed
        .clamp(-max_rotation_speed, max_rotation_speed);

    transform.rotation *= Quat::from_rotation_z(rocket.rotation_speed * time.delta_seconds());

    let direction = transform.rotation * Vec3::Y;
    rocket.velocity = Vec2::new(direction.x, direction.y) * rocket.speed;

    transform.translation += rocket.velocity.extend(0.0) * time.delta_seconds();
}

fn update_rocket_status(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut entities: Query<(Entity, &mut Rocket, &mut Transform)>,
    time: Res<Time>,
) {
    let rockets: Vec<(Entity, Mut<'_, Rocket>, Mut<'_, Transform>)> = entities.iter_mut().collect();

    if rockets.len() > 1 {
        let (entity, rocket1, transform1) = &rockets[0];
        let (entity2, rocket2, transform2) = &rockets[1];

        if check_sun_collision(transform1, rocket1.radius_collision + 30.) {
            commands.entity(*entity).despawn();
        }

        if check_sun_collision(transform2, rocket2.radius_collision + 30.) {
            commands.entity(*entity2).despawn();
        }

        if check_collision(transform1, transform2, rocket1.radius_collision + 100.) {
            for (entity, _, _) in entities.iter() {
                commands.entity(entity).despawn();
            }
        }
    }

    for (_, mut rocket, mut transform) in entities.iter_mut() {
        handle_rocket_movement(&time, &keys, &mut rocket, &mut transform);
    }
}
