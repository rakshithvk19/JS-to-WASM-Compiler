// Test mixed type arithmetic with auto-conversion
function f32_mixed_arithmetic() {
  let a = 5 + 3.14;
  let b = 10.0 - 3;
  let c = 2 * 1.5;
  let result = a + b + c;
  return result;
}

f32_mixed_arithmetic();
