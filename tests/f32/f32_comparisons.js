// Test f32 comparison operations
function f32_comparisons() {
  let a = 3.14 > 2.5;
  let b = 1.0 == 1.0;
  let c = 5.5 <= 10.0;
  let d = 2.0 != 3.0;
  let result = a + b + c + d;
  return result;
}

f32_comparisons();
