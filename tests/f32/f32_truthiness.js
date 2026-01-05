// Test f32 truthiness in boolean contexts
function f32_truthiness() {
  let result = 0;
  
  // 0.0 is falsy
  if (0.0) {
    result = result + 1;
  } else {
    result = result + 10;
  }
  
  // 0.01 is truthy (any non-zero f32)
  if (0.01) {
    result = result + 5;
  } else {
    result = result + 100;
  }
  
  // -0.5 is truthy (non-zero, even negative)
  if (-0.5) {
    result = result + 2;
  }
  
  // Large number is truthy
  if (1000.0) {
    result = result + 3;
  }
  
  return result;
}

f32_truthiness();
