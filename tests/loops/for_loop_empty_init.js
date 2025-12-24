// For loop with empty init - variable declared outside
function for_loop_empty_init() {
  let i = 0;
  let sum = 0;
  for (; i < 5; i = i + 1) {
    sum = sum + i;
  }
  return sum;
}

for_loop_empty_init();
