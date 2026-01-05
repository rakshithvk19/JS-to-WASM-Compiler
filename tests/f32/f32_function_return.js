// Test function returning f32
function getPi() {
  return 3.14159;        // Returns f32
}

function f32_function_return() {
  let pi = getPi();
  let result = pi * 2.0;
  return result;
}

f32_function_return();
