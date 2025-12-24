// Continue statement in for loop - skip multiples of 3
function continue_for() {
  let sum = 0;
  for (let i = 1; i <= 10; i = i + 1) {
    if (i % 3 == 0) {
      continue;
    }
    sum = sum + i;
  }
  return sum;
}

continue_for();
