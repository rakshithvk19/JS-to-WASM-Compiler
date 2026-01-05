// Test constant folding for f32
function f32_constant_folding() {
  let x = 3.0 + 4.0;     // Should fold to 7.0 at compile time
  let y = 10.0 - 2.0;    // Should fold to 8.0 at compile time
  let z = 2.0 * 3.0;     // Should fold to 6.0 at compile time
  let result = x + y + z;
  return result;
}

f32_constant_folding();
