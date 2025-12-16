// Break statement in for loop - sum until threshold
function break_for() {
  let sum = 0;
  for (let i = 1; i <= 100; i = i + 1) {
    sum = sum + i;
    if (sum > 50) {
      break;
    }
  }
  return sum;
}

break_for();
