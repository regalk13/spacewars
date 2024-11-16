@fragment
fn main(@location(0) frag_coord: vec2<f32>) -> @location(0) vec4<f32> {
    return vec4<f32>(0.5, 0.5, 0.5, 1.0); // Gray color
}
