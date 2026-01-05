// Test zero comparisons with auto-conversion
function f32_zero_comparison() {
  let a = 0.0 == 0;      // f32 == i32 -> both convert to f32, should be true (1)
  let b = 0 == 0.0;      // i32 == f32 -> both convert to f32, should be true (1)
  let c = 0.0 != 0;      // Should be false (0)
  let d = 0 < 0.0;       // Should be false (0)
  let e = 0.0 <= 0;      // Should be true (1)
  let result = a + b + c + d + e;
  return result;
}

f32_zero_comparison();
