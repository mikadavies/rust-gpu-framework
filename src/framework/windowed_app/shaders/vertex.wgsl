@vertex
fn main(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4<f32> {
    var x: f32 = 1.0;
    var y: f32 = 1.0;
    switch vertex_index {
        case 0u: {
            x = -1.0;
            y = -1.0;
        }
        case 1u: {
            x = -1.0;
        }
        case 3u: {
            x = -1.0;
            y = -1.0;
        }
        case 4u: {
            y = -1.0;
        }
        default: {}
    }
  return vec4(x,y,0.0,1.0);
}