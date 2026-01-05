// Test all f32 literal formats
function f32_literals() {
  let a = 3.14;
  let b = 3.;
  let c = .5;
  let d = 1e2;
  let e = 2.5e1;
  let result = a + b + c + d + e;
  return result;
}

f32_literals();
