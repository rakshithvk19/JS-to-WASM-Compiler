// Test mixed type logical operators with auto-conversion
function f32_mixed_logical() {
  let a = 5 && 3.14;     // i32 && f32 -> both truthy, returns 3.14 (f32)
  let b = 0.0 || 5;      // f32 || i32 -> first falsy, returns 5 (widened to 5.0)
  let c = 3.14 && 0;     // f32 && i32 -> second is falsy, returns 0 (widened to 0.0)
  let d = 1 || 2.5;      // i32 || f32 -> first truthy, returns 1 (widened to 1.0)
  let result = a + b + c + d;
  return result;
}

f32_mixed_logical();
