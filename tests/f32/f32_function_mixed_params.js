// Test function with mixed parameter types
function add(a, b) {
  return a + b;
}

function f32_function_mixed_params() {
  let result1 = add(5, 3.14);       // First call: params locked as (i32, f32), result is f32
  let result2 = add(10, 2.5);       // Second call: must match (i32, f32)
  let result = result1 + result2;
  return result;
}

f32_function_mixed_params();
