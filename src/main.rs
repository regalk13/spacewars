use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    ecs::query,
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
    transform,
    window::WindowMode,
};

// Rocket component
#[derive(Component)]
struct Rocket {
    speed: f32,
    max_speed: f32,
    velocity: Vec2,
    rotation_speed: f32,
    controls: RocketControls,
}

struct RocketControls {
    rotate_left: KeyCode,
    rotate_right: KeyCode,
    accelerate: KeyCode,
}

#[derive(Component)]
struct Sun {}

// Projectile component
#[derive(Component)]
struct Projectile;

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
                    resolution: (1280., 720.).into(),
                    mode: WindowMode::Windowed,
                    resizable: false,
                    ..default()
                }),
                ..default()
            }),
            Material2dPlugin::<MovingPatternMaterial>::default(),
        ))
        .add_systems(Startup, (setup, add_sun, add_rockets).chain())
        .add_systems(
            Update,
            (
                update_rocket_status,
                gravitational_pull,
                check_collision,
                clip_rockets,
            ),
        )
        .run();
}

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

fn add_rockets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let rocket_image = asset_server.load("Rocket1-v2.png");
    let rocket_image_b = asset_server.load("Rocket1-v3.png");

    let pos = Vec2::new(-550., 250.);
    let pos_b = Vec2::new(550., -250.);
    let direction = Vec2::ZERO - pos;
    let direction_b = Vec2::ZERO - pos_b;
    let angle = direction.y.atan2(direction.x) - std::f32::consts::FRAC_PI_2;
    let angle_b = direction_b.y.atan2(direction_b.x) - std::f32::consts::FRAC_PI_2;

    let rocket_a_pos = Transform {
        translation: Vec3::new(-550., 250.0, 10.0),
        scale: Vec3::splat(0.12),
        rotation: Quat::from_rotation_z(angle),
        ..Default::default()
    };
    let rocket_b_pos = Transform {
        translation: Vec3::new(550., -250.0, 10.0),
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
            max_speed: 200.0,
            velocity: Vec2::ZERO,
            rotation_speed: f32::to_radians(0.0),
            controls: RocketControls {
                rotate_left: KeyCode::KeyA,
                rotate_right: KeyCode::KeyD,
                accelerate: KeyCode::KeyS,
            },
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
            max_speed: 200.0,
            velocity: Vec2::ZERO,
            rotation_speed: f32::to_radians(0.0),
            controls: RocketControls {
                rotate_left: KeyCode::KeyJ,
                rotate_right: KeyCode::KeyL,
                accelerate: KeyCode::KeyK,
            },
        },
    ));
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<MovingPatternMaterial>>,
    window: Query<&Window>,
) {
    let window = window.single();
    let resolution = Vec2::new(window.width(), window.height());

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            ..default()
        },
        BloomSettings::default(),
    ));
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
        rotation_input += 1.0;
    }
    if keys.pressed(rocket.controls.rotate_right) {
        rotation_input -= 1.0;
    }

    let max_rotation_speed = f32::to_radians(70.0);
    let rotation_acceleration = f32::to_radians(50.0 * time.delta_seconds());
    rocket.rotation_speed += rotation_input * rotation_acceleration;
    rocket.rotation_speed = rocket
        .rotation_speed
        .clamp(-max_rotation_speed, max_rotation_speed);

    transform.rotation *= Quat::from_rotation_z(rocket.rotation_speed * time.delta_seconds());

    if keys.pressed(rocket.controls.rotate_right) && keys.pressed(rocket.controls.rotate_left) {
        rocket.rotation_speed = 0.0
    }

    transform.rotation *= Quat::from_rotation_z(rocket.rotation_speed * time.delta_seconds());

    let direction = transform.rotation * Vec3::Y;
    rocket.velocity = Vec2::new(direction.x, direction.y) * rocket.speed;

    transform.translation += rocket.velocity.extend(0.0) * time.delta_seconds();

    //  println!("{:?}", rocket.rotation_speed);
    // println!("{:?}", transform.rotation);
}

fn update_rocket_status(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Rocket, &mut Transform)>,
    time: Res<Time>,
) {
    for (_, (mut rocket, mut transform)) in query.iter_mut().enumerate() {
        handle_rocket_movement(&time, &keys, &mut rocket, &mut transform);
        // transform.translation += rocket.velocity.extend(0.0) * time.delta_seconds();
    }
}

fn are_rockets_colliding(transform_a: &Transform, transform_b: &Transform) -> bool {
    let a_pos = Vec2::new(transform_a.translation.x, transform_a.translation.y);
    let b_pos = Vec2::new(transform_b.translation.x, transform_b.translation.y);

    let a_size = Vec2::splat(50.0);
    let b_size = Vec2::splat(50.0);

    let a_min = a_pos - a_size / 2.0;
    let a_max = a_pos + a_size / 2.0;
    let b_min = b_pos - b_size / 2.0;
    let b_max = b_pos + b_size / 2.0;

    !(a_max.x < b_min.x || a_min.x > b_max.x || a_max.y < b_min.y || a_min.y > b_max.y)
}

fn check_collision(mut query: Query<(&mut Rocket, &Transform)>) {
    let rockets: Vec<(Mut<Rocket>, &Transform)> = query.iter_mut().collect();

    for i in 0..rockets.len() {
        for j in (i + 1)..rockets.len() {
            let (_, transform_a) = &rockets[i];
            let (_, transform_b) = &rockets[j];

            if are_rockets_colliding(transform_a, transform_b) {
                // println!("Collide!");
            }
        }
    }
}

fn clip_rockets(mut query: Query<&mut Transform>) {
    let mut pos: Vec<Mut<Transform>> = query.iter_mut().collect();

    for i in 0..pos.len() {
        let transform = &mut pos[i];
        if transform.translation[0] < -670.0 || transform.translation[0] > 670.0 {
            transform.translation = Vec3::new(
                -transform.translation[0],
                transform.translation[1],
                transform.translation[2],
            );
        }
        if transform.translation[1] < -380.0 || transform.translation[1] > 380.0 {
            transform.translation = Vec3::new(
                transform.translation[0],
                -transform.translation[1],
                transform.translation[2],
            );
        }
    }
}

fn gravitational_pull(mut rocket_query: Query<(&mut Rocket, &mut Transform)>, time: Res<Time>) {
    let sun_position = Vec2::ZERO;
    let gravitational_constant = 120000000.0;

    for (mut rocket, mut transform) in rocket_query.iter_mut() {
        let rocket_position = Vec2::new(transform.translation.x, transform.translation.y);

        let direction = sun_position - rocket_position;
        let distance = direction.length();

        if distance < 70.0 {
            continue;
        }

        let force = gravitational_constant / (distance * distance);

        let acceleration = direction.normalize() * force;

        rocket.velocity += acceleration * time.delta_seconds();

        if rocket.velocity.length() > rocket.max_speed {
            rocket.velocity = rocket.velocity.normalize() * rocket.max_speed;
        }
        transform.translation.x += rocket.velocity.x * time.delta_seconds();
        transform.translation.y += rocket.velocity.y * time.delta_seconds();
    }
}
