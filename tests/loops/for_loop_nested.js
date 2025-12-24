// Nested for loops - multiplication table sum
function for_loop_nested() {
  let sum = 0;
  for (let i = 1; i <= 5; i = i + 1) {
    for (let j = 1; j <= 5; j = j + 1) {
      sum = sum + i * j;
    }
  }
  return sum;
}

for_loop_nested();
