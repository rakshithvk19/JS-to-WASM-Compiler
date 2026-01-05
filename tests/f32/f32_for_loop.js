// Test f32 in for loop
function f32_for_loop() {
  let sum = 0.0;
  
  for (let i = 0.0; i < 5.0; i = i + 1.0) {
    sum = sum + i;
  }
  
  return sum;
}

f32_for_loop();
