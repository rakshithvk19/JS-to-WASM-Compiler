// Error: Type mismatch on assignment
function f32_type_mismatch_assignment() {
  let x = 5;             // x is i32
  x = 3.14;              // ERROR: Cannot assign f32 to i32 variable
  return x;
}

f32_type_mismatch_assignment();
