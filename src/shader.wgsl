struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] color: vec4<f32>;
};

var<private> full: array<vec2<f32>, 6> = array<vec2<f32>, 6>(
    vec2<f32>(-1.0, -1.0),
    vec2<f32>(1.0, -1.0),
    vec2<f32>(1.0, 1.0),
    vec2<f32>(-1.0, -1.0),
    vec2<f32>(1.0, 1.0),
    vec2<f32>(-1.0, 1.0),
);

// Vertex shader

[[stage(vertex)]]
fn vs_main(
    [[builtin(vertex_index)]] vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    out.position = vec4<f32>(full[vertex_index]*vec2<f32>(0.5), 0.0, 1.0);
    out.color = vec4<f32>(1.0, 1.0, 1.0, 1.0);
    return out;
}

// Fragment shader

[[stage(fragment)]]
fn fs_main(
    in: VertexOutput,
) -> [[location(0)]] vec4<f32> {
    return in.color;
}