// Test dead code elimination with f32
function f32_dead_code() {
  let result = 5;
  
  if (0.0) {             // Dead code: 0.0 is falsy, should be eliminated
    let x = 10;
    result = x + 100;
  }
  
  while (0.0) {          // Dead code: loop never executes
    result = result + 50;
  }
  
  return result;
}

f32_dead_code();
