// Test mixed type comparisons with auto-conversion
function f32_mixed_comparison() {
  let a = 5 == 5.0;      // i32 == f32 -> convert to f32, should be true (1)
  let b = 3 < 3.14;      // i32 < f32 -> convert to f32, should be true (1)
  let c = 10.0 >= 10;    // f32 >= i32 -> convert to f32, should be true (1)
  let d = 2.5 != 2;      // f32 != i32 -> convert to f32, should be true (1)
  let result = a + b + c + d;
  return result;
}

f32_mixed_comparison();
