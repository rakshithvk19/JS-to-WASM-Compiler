// Error: Function parameter type mismatch on second call
function identity(a) {
  return a;
}

function f32_function_param_mismatch() {
  let x = identity(5);       // First call: param locked as i32
  let y = identity(3.14);    // ERROR: Second call with f32, expected i32
  return x + y;
}

f32_function_param_mismatch();
