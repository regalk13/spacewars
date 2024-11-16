use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{
            AsBindGroup, ShaderRef, 
    },
        sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};

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
        .add_plugins((DefaultPlugins,  Material2dPlugin::<MovingPatternMaterial>::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, update_position)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<MovingPatternMaterial>>,
    window: Query<&Window>,
) {
    let window = window.single();
    let resolution = Vec2::new(window.width(), window.height());

    commands.spawn(Camera2dBundle::default());
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
