@group(0) @binding(0) var<uniform> u_time: f32;

@vertex
fn main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
   let x = f32(1 - i32(in_vertex_index)) * 0.5;
   let y = f32(1 - i32(in_vertex_index & 1u) * 2) * 0.5;

   let c: f32 = cos(u_time);
   let s: f32 = sin(u_time);
   let R: mat2x2<f32> = mat2x2<f32>(c, s, -s, c);
   var uv: vec2<f32> = R * vec2<f32>( x, y );

   return vec4<f32>(uv.x, uv.y, 0.0, 1.0);
}
