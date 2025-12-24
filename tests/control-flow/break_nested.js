// Nested loops with break - find first pair that sums to target
function break_nested() {
  let result = 0;
  for (let i = 1; i <= 5; i = i + 1) {
    for (let j = 1; j <= 5; j = j + 1) {
      if (i + j == 7) {
        result = i * 10 + j;
        break;
      }
    }
    if (result > 0) {
      break;
    }
  }
  return result;
}

break_nested();
