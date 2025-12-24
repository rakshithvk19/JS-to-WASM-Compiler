// For loop with empty increment - manual increment in body
function for_loop_empty_incr() {
  let sum = 0;
  for (let i = 0; i < 5; ) {
    sum = sum + i;
    i = i + 1;
  }
  return sum;
}

for_loop_empty_incr();
