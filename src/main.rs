use bevy::{
    prelude::*,
    reflect::TypePath,
      core_pipeline::{
        bloom::BloomSettings,
        tonemapping::Tonemapping,
    },
    render::render_resource::{
            AsBindGroup, ShaderRef, 
    },
        sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
    window::WindowMode,
};

// Rocket component
#[derive(Component)]
struct Rocket {
    speed: f32,
    max_speed: f32,
    velocity: Vec2,
    rotation_speed: f32,
    move_timer: Timer,
}

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
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
              title: "SpaceWars".into(),
              name: Some("bevy.app".into()),
              resolution: (1280., 720.).into(),
              mode: WindowMode::Windowed,
              resizable: false,
              ..default()
        }),
        ..default()
        }),  Material2dPlugin::<MovingPatternMaterial>::default()))
        .add_systems(Startup, (setup, add_sun, add_rockets).chain())
        .add_systems(Update, rocket_movement)
        .run();
}

fn add_sun(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials:  ResMut<Assets<ColorMaterial>>,
) {

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(Circle::new(50.0))).into(),
        material: materials.add(Color::srgb(7.0, 7.0, 0.0)),
        transform: Transform::from_xyz(0.0, 0.0, 3.0),
        ..default()
    });

}

fn add_rockets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let rocket_image = asset_server.load("Rocket1-v2.png");
    

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
        scale: Vec3::splat(0.12),
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
            speed: 0.0, // Initial speed
            max_speed: 500.0, // Max speed
            velocity: Vec2::ZERO,
            rotation_speed: f32::to_radians(360.0), 
            move_timer: Timer::from_seconds(0.2, TimerMode::Repeating),
        },  
    ));
    commands.spawn((
        SpriteBundle {
            texture: rocket_image,
            transform: rocket_b_pos,
            ..default()
        },
        Rocket {
            speed: 0.0,
            max_speed: 500.0,
            velocity: Vec2::ZERO,
            rotation_speed: f32::to_radians(360.0), 
            move_timer: Timer::from_seconds(0.2, TimerMode::Repeating),
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
        mesh: meshes.add(Mesh::from(Rectangle::from_size(resolution))).into(),
        material: materials.add(MovingPatternMaterial {
            color: LinearRgba::WHITE,
            time: 0.0
        }),
        ..default()
    });
}

fn update_position(
    mut materials: ResMut<Assets<MovingPatternMaterial>>,
    time: Res<Time>,
    query: Query<&Handle<MovingPatternMaterial>>,
) {
  for handle in query.iter() {
        // Access the material using the handle
        if let Some(material) = materials.get_mut(handle) {
            // Update the `time` field
            material.time = time.elapsed_seconds() as f32;
            println!("${}", material.time)
        }
    }
}

fn rocket_movement(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Rocket, &mut Transform)>,
    time: Res<Time>,
) {
    if let Some((mut rocket, mut transform)) = query.iter_mut().next() {
        if keys.pressed(KeyCode::KeyD) {
            if rocket.rotation_speed > -2. {
                rocket.rotation_speed -= 2.0 * time.delta_seconds();
            }
        }
        if keys.pressed(KeyCode::KeyA) {
            if rocket.rotation_speed < 2. {
                rocket.rotation_speed += 2.0 * time.delta_seconds();
            }
        }
        
        if keys.pressed(KeyCode::KeyS) {
            if rocket.speed < rocket.max_speed {
                rocket.speed += 2.0 * time.delta_seconds();
            }
        } else {
            if rocket.speed > 0.0 {
                rocket.speed -= 2.0 * time.delta_seconds();
            }
        }


        if keys.pressed(KeyCode::KeyA) && keys.pressed(KeyCode::KeyD) { 
           rocket.rotation_speed = 0.02;
        }
        if !keys.pressed(KeyCode::KeyA) && !keys.pressed(KeyCode::KeyD) {
            
            if rocket.rotation_speed > 0.02 {
                rocket.rotation_speed -= 5.0 * time.delta_seconds(); 
            } else if rocket.rotation_speed < 0.02 {
                rocket.rotation_speed += 5.0 * time.delta_seconds();
            }
        }
        
        transform.rotation *= Quat::from_rotation_z(rocket.rotation_speed * time.delta_seconds());

        let direction = transform.rotation * Vec3::Y; 
        rocket.velocity = Vec2::new(direction.x, direction.y) * rocket.speed;

        transform.translation += rocket.velocity.extend(0.0) * time.delta_seconds();
    }
    
    if let Some((mut rocket, mut transform)) = query.iter_mut().skip(1).next() {
        
        if keys.pressed(KeyCode::KeyL) {
            if rocket.rotation_speed > -2.0 {
                rocket.rotation_speed -= 2.0 * time.delta_seconds();
            }
        }
        if keys.pressed(KeyCode::KeyJ) {
            if rocket.rotation_speed < 2.0 {
                rocket.rotation_speed += 2.0 * time.delta_seconds();
            }
        }

        if keys.pressed(KeyCode::KeyJ) && keys.pressed(KeyCode::KeyL) { 
           rocket.rotation_speed = 0.02;
        }
        if !keys.pressed(KeyCode::KeyJ) && !keys.pressed(KeyCode::KeyL) {
            
            if rocket.rotation_speed > 0.02 {
                rocket.rotation_speed -= 5.0 * time.delta_seconds(); 
            } else if rocket.rotation_speed < 0.02 {
                rocket.rotation_speed += 5.0 * time.delta_seconds();
            }
        }
        
        if keys.pressed(KeyCode::KeyK) {
            if rocket.speed < rocket.max_speed {
                rocket.speed += 2.0 * time.delta_seconds();
            }
        } else {
            if rocket.speed > 0.0 {
                rocket.speed -= 2.0 * time.delta_seconds();
            }
        }
        transform.rotation *= Quat::from_rotation_z(rocket.rotation_speed * time.delta_seconds());

        let direction = transform.rotation * Vec3::Y; 
        rocket.velocity = Vec2::new(direction.x, direction.y) * rocket.speed;

        transform.translation += rocket.velocity.extend(0.0) * time.delta_seconds();
    }
}
