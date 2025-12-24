// For loop counting down
function for_loop_countdown() {
  let sum = 0;
  for (let i = 10; i > 0; i = i - 1) {
    sum = sum + i;
  }
  return sum;
}

for_loop_countdown();
