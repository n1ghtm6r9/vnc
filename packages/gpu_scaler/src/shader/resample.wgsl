struct Params {
    origin: vec2<i32>,
    input_size: vec2<i32>,
    output_size: vec2<i32>,
    encode_srgb: u32,
    padding: u32,
}

@group(0) @binding(0) var input_texture: texture_2d<f32>;
@group(0) @binding(1) var<uniform> params: Params;

fn catmull_rom(x: f32) -> f32 {
    let a = abs(x);
    if a < 1.0 {
        return 1.5 * a * a * a - 2.5 * a * a + 1.0;
    }
    if a < 2.0 {
        return -0.5 * a * a * a + 2.5 * a * a - 4.0 * a + 2.0;
    }
    return 0.0;
}

fn srgb_encode(linear: vec3<f32>) -> vec3<f32> {
    let clamped = clamp(linear, vec3<f32>(0.0), vec3<f32>(1.0));
    let low = clamped * 12.92;
    let high = 1.055 * pow(clamped, vec3<f32>(1.0 / 2.4)) - vec3<f32>(0.055);
    return select(high, low, clamped <= vec3<f32>(0.0031308));
}

fn finish(linear: vec3<f32>) -> vec3<f32> {
    if params.encode_srgb != 0u {
        return srgb_encode(linear);
    }
    return linear;
}

fn output_pixel(position: vec4<f32>) -> vec2<i32> {
    return vec2<i32>(position.xy) - params.origin;
}

fn resample(pixel: vec2<i32>, vertical: bool) -> vec3<f32> {
    let along = select(pixel.x, pixel.y, vertical);
    let across = select(pixel.y, pixel.x, vertical);
    let input_length = select(params.input_size.x, params.input_size.y, vertical);
    let output_length = select(params.output_size.x, params.output_size.y, vertical);

    let scale = f32(output_length) / f32(input_length);
    let support = max(1.0 / scale, 1.0);
    let centre = (f32(along) + 0.5) / scale - 0.5;
    let first = i32(floor(centre - 2.0 * support)) + 1;
    let last = i32(floor(centre + 2.0 * support));

    var sum = vec3<f32>(0.0);
    var weight_sum = 0.0;
    for (var tap = first; tap <= last; tap++) {
        let weight = catmull_rom((f32(tap) - centre) / support);
        let index = clamp(tap, 0, input_length - 1);
        let coordinates = select(vec2<i32>(index, across), vec2<i32>(across, index), vertical);
        sum += textureLoad(input_texture, coordinates, 0).rgb * weight;
        weight_sum += weight;
    }
    return sum / weight_sum;
}

@vertex
fn vs_main(@builtin(vertex_index) index: u32) -> @builtin(position) vec4<f32> {
    let corner = vec2<f32>(f32((index << 1u) & 2u), f32(index & 2u));
    return vec4<f32>(corner * 2.0 - vec2<f32>(1.0), 0.0, 1.0);
}

@fragment
fn fs_horizontal(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    return vec4<f32>(resample(output_pixel(position), false), 1.0);
}

@fragment
fn fs_vertical(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    return vec4<f32>(finish(resample(output_pixel(position), true)), 1.0);
}

@fragment
fn fs_nearest(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    let pixel = output_pixel(position);
    let source = ((pixel * 2 + vec2<i32>(1)) * params.input_size) / (params.output_size * 2);
    return vec4<f32>(finish(textureLoad(input_texture, source, 0).rgb), 1.0);
}
