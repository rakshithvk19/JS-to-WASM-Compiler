// Error: Unexpected token
function unexpected_token() {
  let x = 10 @;
  return x;
}

unexpected_token();
