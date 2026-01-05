// Test function with f32 parameters (first-call type inference)
function multiply(a, b) {
  return a * b;
}

function f32_function_params() {
  let result1 = multiply(2.5, 4.0);   // First call: params locked as (f32, f32)
  let result2 = multiply(3.0, 5.0);   // Second call: must match (f32, f32)
  let result = result1 + result2;
  return result;
}

f32_function_params();
