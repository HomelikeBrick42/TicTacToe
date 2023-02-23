struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) tex_coord: vec2<f32>,
    @location(2) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) position: vec2<f32>,
    @location(1) tex_coord: vec2<f32>,
    @location(2) color: vec3<f32>,
};

struct Camera {
    position: vec2<f32>,
    screen_size: vec2<f32>,
    scale: f32,
};

@group(0)
@binding(0)
var<uniform> camera: Camera;

const CIRCLE_WIDTH: f32 = 0.1;

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    let aspect = f32(camera.screen_size.x) / f32(camera.screen_size.y);

    var out: VertexOutput;
    out.position = model.position;
    out.clip_position = vec4<f32>((out.position - camera.position) * camera.scale / vec2<f32>(aspect, 1.0), 0.0, 1.0);
    out.tex_coord = model.tex_coord;
    out.color = model.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.tex_coord * 2.0 - 1.0;

    if (abs(length(uv) - (1.0 - CIRCLE_WIDTH * 2.0)) > CIRCLE_WIDTH) {
        discard;
    }

    return vec4<f32>(in.color, 1.0);
}
