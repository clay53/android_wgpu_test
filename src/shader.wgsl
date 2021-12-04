struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] color: vec4<f32>;
};

// Vertex shader

[[stage(vertex)]]
fn vs_main(
    [[builtin(vertex_index)]] vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;

    switch (vertex_index) {
        case 0, 3: {
            out.position = vec4<f32>(-0.5, -0.5, 0.0, 1.0);
        }
        case 1: {
            out.position = vec4<f32>(0.5, -0.5, 0.0, 1.0);
        }
        case 2, 4: {
            out.position = vec4<f32>(0.5, 0.5, 0.0, 1.0);
        }
        case 5: {
            out.position = vec4<f32>(-0.5, 0.5, 0.0, 1.0);
        }
    }

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