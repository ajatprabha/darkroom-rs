struct Kernel {
    sum: f32,
    size: u32,
    values: array<f32>,
};

@group(0) @binding(0) var<storage, read> kernel: Kernel;
@group(1) @binding(0) var input_texture: texture_2d<f32>;
@group(1) @binding(1) var output_texture: texture_storage_2d<rgba8unorm, write>;

@compute @workgroup_size(8, 8, 1)
fn main(@builtin(global_invocation_id) GlobalInvocationID: vec3<u32>) {
    let x: u32 = GlobalInvocationID.x;
    let y: u32 = GlobalInvocationID.y;
    let dimensions: vec2<u32> = textureDimensions(input_texture);

    if (x >= dimensions.x || y >= dimensions.y) {
        return;
    }

    let kernel_size: i32 = i32(kernel.size);
    let half_size: i32 = kernel_size / 2;

    var color: vec4<f32> = vec4<f32>(0.0);

    for (var i: i32 = -half_size; i <= half_size; i = i + 1) {
        for (var j: i32 = -half_size; j <= half_size; j = j + 1) {
            let nx: i32 = i32(x) + i;
            let ny: i32 = i32(y) + j;

            if (nx >= 0 && nx < i32(dimensions.x) && ny >= 0 && ny < i32(dimensions.y)) {
                let pixel: vec4<f32> = textureLoad(input_texture, vec2<i32>(nx, ny), 0);
                let kernel_value: f32 = kernel.values[(i + half_size) * kernel_size + (j + half_size)];
                color = color + kernel_value * pixel;
            }
        }
    }

    color = color / kernel.sum;
    textureStore(output_texture, vec2<i32>(i32(x), i32(y)), color);
}
