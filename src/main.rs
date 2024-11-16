use bevy::prelude::*;


const SHADER_ASSET_PATH: &str = "shaders/custom_stars.wgsl";


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
      //  .add_plugins(Startup, setup)
        .run();
}

