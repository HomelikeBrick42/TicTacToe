struct VertexInput {
    @location(0) object_position: vec2<f32>,
    @location(1) scale: vec2<f32>,
    @location(2) is_circle: u32,
    @location(3) circle_width: f32,
    @location(4) position: vec2<f32>,
    @location(5) tex_coord: vec2<f32>,
    @location(6) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @interpolate(flat) @location(0) is_circle: u32,
    @interpolate(flat) @location(1) circle_width: f32,
    @location(2) position: vec2<f32>,
    @location(3) tex_coord: vec2<f32>,
    @location(4) color: vec3<f32>,
};

struct Camera {
    position: vec2<f32>,
    screen_size: vec2<f32>,
    scale: f32,
};

@group(0)
@binding(0)
var<uniform> camera: Camera;

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    let aspect = f32(camera.screen_size.x) / f32(camera.screen_size.y);

    var out: VertexOutput;
    out.is_circle = model.is_circle;
    out.circle_width = model.circle_width;
    out.position = model.position * model.scale + model.object_position;
    out.clip_position = vec4<f32>((out.position - camera.position) * camera.scale / vec2<f32>(aspect, 1.0), 0.0, 1.0);
    out.tex_coord = model.tex_coord;
    out.color = model.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.tex_coord * 2.0 - 1.0;

    if (in.is_circle != 0u && abs(length(uv) - (1.0 - in.circle_width * 2.0)) > in.circle_width) {
        discard;
    }

    return vec4<f32>(in.color, 1.0);
}
