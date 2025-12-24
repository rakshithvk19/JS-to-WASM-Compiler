// Continue statement in while loop - sum only odd numbers
function continue_while() {
  let i = 0;
  let sum = 0;
  while (i < 10) {
    i = i + 1;
    if (i % 2 == 0) {
      continue;
    }
    sum = sum + i;
  }
  return sum;
}

continue_while();
