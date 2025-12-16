// Break statement in while loop - find first number divisible by 7
function break_while() {
  let i = 1;
  let result = 0;
  while (i < 100) {
    if (i % 7 == 0) {
      result = i;
      break;
    }
    i = i + 1;
  }
  return result;
}

break_while();
