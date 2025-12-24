// Error: Continue outside of loop
function continue_outside_loop() {
  let x = 10;
  if (x > 5) {
    continue;
  }
  return x;
}

continue_outside_loop();
