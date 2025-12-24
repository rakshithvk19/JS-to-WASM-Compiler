// Error: Break outside of loop
function break_outside_loop() {
  let x = 10;
  if (x > 5) {
    break;
  }
  return x;
}

break_outside_loop();
