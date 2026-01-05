// Test f32 in while loop condition
function f32_while_loop() {
  let counter = 5.0;
  let sum = 0.0;
  
  while (counter > 0.0) {
    sum = sum + counter;
    counter = counter - 1.0;
  }
  
  return sum;
}

f32_while_loop();
