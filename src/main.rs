use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    reflect::TypePath,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle, Material2d},
    render::{
        mesh::MeshVertexBufferLayoutRef,
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
        },
    },
    color::palettes::basic::PURPLE
};

/* custom material */
#[derive(Asset, TypePath, AsBindGroup, Clone)]
pub struct MovingPatternMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[uniform(1)]
    u_time: f32, // Pass time to the shader
    #[uniform(2)]
    u_resolution: Vec2, // Pass screen resolution to the shader
}


impl Material for MovingPatternMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/stars_materia.vert".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/stars_materia.frag".into()
    }


    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.vertex.entry_point = "main".into();
        descriptor.fragment.as_mut().unwrap().entry_point = "main".into();
        Ok(())
    }
}



const SHADER_ASSET_PATH: &str = "shaders/custom_stars.wgsl";

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>
) {
    let window = window.single();
    commands.spawn(Camera2dBundle::default());
    commands.spawn(MaterialMesh2dBundle {
        // mesh: meshes.add(Rectangle::default()).into(),
        mesh: meshes.add(Rectangle::from_size(Vec2::new(window.width(), window.height()))).into(),
        // transform: Transform::from_xyz(0.0, 0.0, 0.0),
        transform: Transform::default().with_scale(Vec3::splat(128.)),
        material: materials.add(Color::from(PURPLE)),
        ..default()
    });
}

