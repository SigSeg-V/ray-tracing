@group(0) @binding(0)
var<uniform> vertex_uniforms: VertexUniforms;

@group(1) @binding(0) var<uniform> IMAGE_DIMENSIONS: vec3<u32>;
@group(1) @binding(1) var<storage, read_write> IMAGE_BUFFER: array<array<f32, 3>>;


struct VertexOut {
    @builtin(position) position: vec4<f32>,
    @location(0) texture_coord: vec2<f32>,
};

struct VertexUniforms {
    viewProjectionMat: mat4x4<f32>,
    modelMat: mat4x4<f32>,
}

struct VertexIn {
    @location(0) position: vec2<f32>,
    @location(1) texture_coord: vec2<f32>,
}

var<private> v_positions: array<vec2<f32>, 6> = array<vec2<f32>, 6>(
        vec2<f32>( 1.0,  1.0),
        vec2<f32>( 1.0, -1.0),
        vec2<f32>(-1.0, -1.0),
        vec2<f32>( 1.0,  1.0),
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(-1.0,  1.0),
    
);

var<private> v_colors: array<vec4<f32>, 6> = array<vec4<f32>, 6>(
    vec4<f32>(1.0, 0.0, 0.0, 1.0),
    vec4<f32>(1.0, 1.0, 0.0, 1.0),
    vec4<f32>(0.0, 1.0, 0.0, 1.0),
    vec4<f32>(1.0, 0.0, 0.0, 1.0),
    vec4<f32>(0.0, 1.0, 0.0, 1.0),
    vec4<f32>(0.0, 0.0, 0.0, 1.0),
);

@vertex
fn vs_main(in: VertexIn) -> VertexOut {
    return VertexOut(
        vertex_uniforms.viewProjectionMat * vertex_uniforms.modelMat * vec4<f32>(in.position, 0.0, 1.0),
        in.texture_coord
    );
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    let u = in.texture_coord.x;
    let v = in.texture_coord.y;

    let x = u32(u * f32(IMAGE_DIMENSIONS.x));
    let y = u32(v * f32(IMAGE_DIMENSIONS.y));

    let idx = IMAGE_DIMENSIONS.x * y + x;

    var rng_state = init_rng(vec2u(x, y), IMAGE_DIMENSIONS.xy, IMAGE_DIMENSIONS.z);

    return vec4f(rng_next_float(&rng_state), rng_next_float(&rng_state), rng_next_float(&rng_state), 1.0);
 }

fn rng_next_float(state: ptr<function, u32>) -> f32 {
    rng_next_uint(state);
    return f32(*state) / f32(0xffffffffu);
}

fn rng_next_uint(state: ptr<function, u32>) {
    // based on the shadertoy impl of xorshift*32 prng

    let old_state = *state + 747796405u + 2891336453u;
    let word = ((old_state >> ((old_state >> 28u) + 4u)) ^ old_state) * 277803737u;
    *state = (word >> 22u) ^ word;
}

fn init_rng(pixel: vec2u, resolution: vec2u, frame: u32) -> u32 {
    let seed = dot(pixel, vec2u(1u, resolution.x)) ^ jenkins_hash(frame);
    return jenkins_hash(seed);
}

fn jenkins_hash(input: u32) -> u32 {
    var x = input;
    x += x << 10u;
    x ^= x >> 6u;
    x += x << 3u;
    x ^= x >> 11u;
    x += x << 15u;

    return x;
}