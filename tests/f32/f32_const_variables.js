// Test const variables with f32
function f32_const_variables() {
  const pi = 3.14159;
  const radius = 5.0;
  let circumference = 2.0 * pi * radius;
  let area = pi * radius * radius;
  let result = circumference + area;
  return result;
}

f32_const_variables();
