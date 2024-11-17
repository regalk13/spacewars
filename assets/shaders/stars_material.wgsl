#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<uniform> material_color: vec4<f32>;
@group(2) @binding(1) var<uniform> time: f32;

fn random2(p: vec2<f32>) -> f32 {
    let dot_product = dot(p, vec2<f32>(12.9898, 78.233)); // This is the random function
    return fract(sin(dot_product) * 43758.5453);  // Generates pseudo-random values
}

@fragment
fn fragment(vertex: VertexOutput) -> @location(0) vec4<f32> {
    let uv = vertex.position.xy;

    let motion_offset = vec2<f32>(time * 0.1, 0.0);

    let scaled_uv = uv * 0.1; // Further decrease the scale for much larger stars

    let brightness = step(0.995, random2(scaled_uv + motion_offset)); // Increase threshold to 0.995 to drastically reduce stars

    let density_factor = step(0.05, fract(scaled_uv.x) * fract(scaled_uv.y)); // Lower density factor for fewer stars

    return material_color * vec4<f32>(brightness * density_factor, brightness * density_factor, brightness * density_factor, 1.0);
}
