// Nested loops with continue - skip even numbers in inner loop
function continue_nested() {
  let sum = 0;
  for (let i = 1; i <= 3; i = i + 1) {
    for (let j = 1; j <= 4; j = j + 1) {
      if (j % 2 == 0) {
        continue;
      }
      sum = sum + j;
    }
  }
  return sum;
}

continue_nested();
