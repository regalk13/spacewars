#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

const CURVATURE: f32 = 4.2;
const BLUR: f32 = 0.021;
const CA_AMT: f32 = 1.000;
const PIXEL_SIZE: vec2<f32> = vec2<f32>(4.0, 4.0); // Pixel size for pixelation

struct PostProcessSettings {
    intensity: f32,
#ifdef SIXTEEN_BYTE_ALIGNMENT
    // WebGL2 structs must be 16-byte aligned.
    _webgl2_padding: vec3<f32>
#endif
}
@group(0) @binding(2) var<uniform> settings: PostProcessSettings;

fn curveRemapUV(uv: vec2<f32>) -> vec2<f32> {
    var crtUV = uv * 2.0 - vec2<f32>(1.0, 1.0);
    let offset = crtUV.yx / CURVATURE;
    crtUV += crtUV * offset * offset;
    crtUV = crtUV * 0.5 + vec2<f32>(0.5, 0.5);
    return crtUV;
}

fn edgeMask(uv: vec2<f32>) -> f32 {
    let edge = smoothstep(0.0, BLUR, uv.x) * (1.0 - smoothstep(1.0 - BLUR, 1.0, uv.x)) *
               smoothstep(0.0, BLUR, uv.y) * (1.0 - smoothstep(1.0 - BLUR, 1.0, uv.y));
    return edge;
}

fn applyPixelation(uv: vec2<f32>, resolution: vec2<f32>) -> vec2<f32> {
    let pixelUV = floor(uv * resolution / PIXEL_SIZE) * PIXEL_SIZE / resolution;
    return pixelUV;
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let offset_strength = settings.intensity;

    var uv = in.uv;
    let crtUV = curveRemapUV(uv);

    let pixelUV = applyPixelation(crtUV, vec2<f32>(1280.0, 720.0)); // Set resolution for pixelation

    let edge = edgeMask(pixelUV);

    let redChannel = textureSample(screen_texture, texture_sampler, (pixelUV - vec2<f32>(0.5)) * CA_AMT + vec2<f32>(0.5)).r;
    let greenChannel = textureSample(screen_texture, texture_sampler, pixelUV).g;
    let blueChannel = textureSample(screen_texture, texture_sampler, (pixelUV - vec2<f32>(0.5)) / CA_AMT + vec2<f32>(0.5)).b;

    var color = vec3<f32>(redChannel, greenChannel, blueChannel) * edge;

    let fragCoord = in.position.xy;  
    if (fragCoord.y % 2.0 < 1.0) {
        color *= 0.8;
    }


    let noise = fract(sin(dot(fragCoord, vec2<f32>(12.9898, 78.233))) * 758.53);
    color += noise * 0.02;

    return vec4<f32>(color, 1.0);
}