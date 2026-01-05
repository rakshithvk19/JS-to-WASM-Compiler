// Error: Inconsistent return types
function f32_inconsistent_return(x) {
  if (x > 0) {
    return 5;            // Returns i32
  }
  return 3.14;           // ERROR: Returns f32, inconsistent with first return
}

function test() {
  return f32_inconsistent_return(10);
}

test();
