// Test negative f32 values
function f32_negative() {
  let a = -3.14;
  let b = -(-2.5);
  let c = -(1.0);
  let result = a + b + c;
  return result;
}

f32_negative();
