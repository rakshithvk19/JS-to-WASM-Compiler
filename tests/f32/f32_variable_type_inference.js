// Test variable type inference with f32
function f32_variable_type_inference() {
  let x = 3.14;          // x inferred as f32
  let y = x + 2.0;       // y also f32 (f32 + f32)
  let z = 5;             // z is i32
  let w = z + x;         // w is f32 (i32 + f32 -> auto-convert)
  let result = x + y + w;
  return result;
}

f32_variable_type_inference();
