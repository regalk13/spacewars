// Import necessary vertex output
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

// Uniforms
@group(2) @binding(0) var<uniform> material_color: vec4<f32>;
@group(2) @binding(1) var<uniform> time: f32;

// Pseudo-random function for star pattern
fn random2(p: vec2<f32>) -> f32 {
    let dot_product = dot(p, vec2<f32>(12.9898, 78.233));
    return fract(sin(dot_product) * 43758.5453);
}

// Main fragment shader
@fragment
fn fragment(vertex: VertexOutput) -> @location(0) vec4<f32> {
    let uv = vertex.position.xy;

    // Offset based on time to create motion
    let motion_offset = vec2<f32>(time * 0.1, 0.0);

    // Generate random brightness for stars
    let brightness = step(0.99, random2(uv + motion_offset));

    // Combine material color and brightness
    return material_color * vec4<f32>(brightness, brightness, brightness, 1.0);
}

