@group(0) @binding(0) var texture: texture_storage_1d<rgba8unorm, write>;

@compute @workgroup_size(32)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    let index = id.x;

    // Random movement
    let star_pos = vec2<f32>(
        fract(sin(index * 12.9898) * 43758.5453),
        fract(cos(index * 4.1414) * 32844.1234)
    );

    let brightness = fract(sin(index * 1.234) * 78965.4321);

    texture.store(index, vec4<f32>(star_pos.x, star_pos.y, brightness, 1.0));
}
