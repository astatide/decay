// Vertex shader

// CAMERA!
struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0) // 1.
var<uniform> camera: CameraUniform;

// TIME!
struct TimeUniform {
    time: vec4<f32>,
};
@group(1) @binding(0)
var<uniform> time: TimeUniform;

// VERTICES!
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

// instances!
struct InstanceInput {
    @location(5) model_matrix_0: vec4<f32>,
    @location(6) model_matrix_1: vec4<f32>,
    @location(7) model_matrix_2: vec4<f32>,
    @location(8) model_matrix_3: vec4<f32>,
};


@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput
) -> VertexOutput {
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );
    var out: VertexOutput;
    out.color = model.color;
    // out.tex_coords = model.tex_coords;
    // We'll apply the model_matrix before we apply camera_uniform.view_proj. We do this because the camera_uniform.view_proj changes the coordinate system from world space to camera space. Our model_matrix is a world space transformation, so we don't want to be in camera space when using it.
    var pos = model_matrix * vec4<f32>(model.position, 1.0); //+ time.time;
    out.clip_position = camera.view_proj * pos; // 2.
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
