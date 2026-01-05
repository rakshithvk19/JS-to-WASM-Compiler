// Error: Modulo operation not supported for f32
function f32_modulo_error() {
  let x = 5.0 % 3.0;     // ERROR: Modulo not supported for f32 types
  return x;
}

f32_modulo_error();
