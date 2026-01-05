// Test chained operations with mixed types
function f32_chained_operations() {
  let a = 5 + 3.14 * 2.0 - 1;         // 5 + 6.28 - 1 = 10.28 (f32)
  let b = 10.0 / 2.0 + 3 * 2;         // 5.0 + 6.0 = 11.0 (f32)
  let c = 2 * 3 + 1.5;                // 6 + 1.5 = 7.5 (f32)
  let d = 100.0 - 50 / 2 + 3.14;      // 100.0 - 25.0 + 3.14 = 78.14 (f32)
  let result = a + b + c + d;
  return result;
}

f32_chained_operations();
