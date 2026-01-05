// Test f32 in if condition (truthiness)
function f32_if_condition() {
  let result = 0;
  
  if (3.14) {            // 3.14 is truthy
    result = result + 10;
  }
  
  if (0.0) {             // 0.0 is falsy
    result = result + 100;
  } else {
    result = result + 5;
  }
  
  if (-2.5) {            // -2.5 is truthy (non-zero)
    result = result + 3;
  }
  
  return result;
}

f32_if_condition();
