struct Conversion {
    origin: vec2<i32>,
    chroma_shift: u32,
    interleaved_chroma: u32,
    rows: array<vec4<f32>, 3>,
    primaries: array<vec4<f32>, 3>,
}

@group(0) @binding(0) var luma: texture_2d<f32>;
@group(0) @binding(1) var chroma_blue: texture_2d<f32>;
@group(0) @binding(2) var chroma_red: texture_2d<f32>;
@group(0) @binding(3) var<uniform> conversion: Conversion;

fn srgb_decode(encoded: vec3<f32>) -> vec3<f32> {
    let low = encoded / 12.92;
    let high = pow((encoded + vec3<f32>(0.055)) / 1.055, vec3<f32>(2.4));
    return select(high, low, encoded <= vec3<f32>(0.04045));
}

fn transform(rows: array<vec4<f32>, 3>, value: vec4<f32>) -> vec3<f32> {
    return vec3<f32>(dot(rows[0], value), dot(rows[1], value), dot(rows[2], value));
}

@vertex
fn vs_main(@builtin(vertex_index) index: u32) -> @builtin(position) vec4<f32> {
    let corner = vec2<f32>(f32((index << 1u) & 2u), f32(index & 2u));
    return vec4<f32>(corner * 2.0 - vec2<f32>(1.0), 0.0, 1.0);
}

@fragment
fn fs_convert(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    let pixel = vec2<i32>(position.xy) - conversion.origin;
    let chroma_pixel = pixel >> vec2<u32>(conversion.chroma_shift);
    let red = textureLoad(chroma_red, chroma_pixel, 0);
    let sample = vec4<f32>(
        textureLoad(luma, pixel, 0).r,
        textureLoad(chroma_blue, chroma_pixel, 0).r,
        select(red.r, red.g, conversion.interleaved_chroma != 0u),
        1.0,
    );
    let encoded = clamp(transform(conversion.rows, sample), vec3<f32>(0.0), vec3<f32>(1.0));
    let linear = transform(conversion.primaries, vec4<f32>(srgb_decode(encoded), 0.0));
    return vec4<f32>(clamp(linear, vec3<f32>(0.0), vec3<f32>(1.0)), 1.0);
}
