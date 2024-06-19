@fragment
fn main(@builtin(position) pixel_in: vec4<f32>) -> @location(0) vec4<f32> {   
    let pixel: vec2<f32> = pixel_in.xy;
    var color: vec4<f32> = vec4(0.0,0.0,0.0,0.0);

    color = pixel_pos(pixel);

    return color;
}

fn pixel_pos(pixel: vec2<f32>) -> vec4<f32> {
    let inverse_distance: f32 = 1.0 / length(pixel);
    return vec4(
        5.0 * inverse_distance,
        1.0 * inverse_distance,
        2.0 * inverse_distance,
        1.0
    );
}
