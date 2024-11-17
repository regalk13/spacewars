#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<uniform> material_color: vec4<f32>;
@group(2) @binding(1) var<uniform> time: f32;

fn random2(p: vec2<f32>) -> f32 {
    let dot_product = dot(p, vec2<f32>(12.9898, 78.233));
    return fract(sin(dot_product) * 43758.5453);
}

@fragment
fn fragment(vertex: VertexOutput) -> @location(0) vec4<f32> {
    let uv = vertex.position.xy;

    let motion_offset = vec2<f32>(0.01, 0.0);

    let scaled_uv = uv * 0.2; // Further decrease the scale for much larger stars

    let brightness = step(0.995, random2(scaled_uv + motion_offset));
    let density_factor = step(0.05, fract(scaled_uv.x) * fract(scaled_uv.y)); 

    return material_color * vec4<f32>(brightness * density_factor, brightness * density_factor, brightness * density_factor, 0.8);
}
