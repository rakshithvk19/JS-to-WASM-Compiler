// Factorial using for loop instead of while
function for_loop_factorial(n) {
  let result = 1;
  for (let i = 1; i <= n; i = i + 1) {
    result = result * i;
  }
  return result;
}

for_loop_factorial(6);
