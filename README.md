# JS to WASM Compiler

A mini JavaScript to WebAssembly Text Format (.wat) compiler written in Rust. Zero dependencies.

## Requirements

- Rust (cargo)
- [wasmtime](https://wasmtime.dev/) for running the output
- Makefile

## Build

```bash
make build
```

## Usage

```bash
./target/release/compiler input.js > output.wat

# Or via make
make run FILE=input.js > output.wat
```

## Running the Output

```bash
wasmtime output.wat --invoke _start
```

## Testing

Tests assert expected values automatically and fail on mismatch.

```bash
make test          # Run all tests
make test-fact     # Factorial (expects 120)
make test-gcd      # GCD (expects 6)
make test-ack      # Ackermann (expects 125)
make test-const    # Const reassignment error
make test-fold     # Constant folding optimization
make test-dead     # Dead code elimination
```

## Architecture

![Architecture Diagram](architecture.png)

```
src/
├── main.rs       # CLI entry point
├── lexer.rs      # Tokenization
├── ast.rs        # AST node definitions
├── parser.rs     # Recursive descent parser
├── optimizer.rs  # Constant folding & dead code elimination
└── codegen.rs    # WAT code generation
```

### Pipeline

```
JS Source → Lexer → Tokens → Parser → AST → Optimizer → CodeGen → WAT
```

## Optimizations

### Constant Folding
Evaluates constant expressions at compile time.

```javascript
let x = 3 + 4 * 2;  // Compiled as: i32.const 11
```

### Dead Code Elimination
Removes unreachable code after `return` statements and eliminates `if(0)` / `while(0)` blocks.

```javascript
function test() {
  return 5;
  let x = 10;  // Eliminated - unreachable
}
```

### Const Immutability
Enforces `const` variables cannot be reassigned.

```javascript
const x = 10;
x = 20;  // Compiler error: Cannot reassign const variable 'x'
```

## Source Location Comments

Generated WAT includes comments mapping instructions to original JS line numbers for debugging.

```wat
;; line 3
i32.const 5
local.set $x
```

## Test Programs

| Test | Description | Expected |
|------|-------------|----------|
| `factorial.js` | Iterative factorial | 120 |
| `gcd.js` | Euclidean GCD | 6 |
| `ackermann.js` | Recursive Ackermann(3,4) | 125 |
| `const_error.js` | Const reassignment detection | Panic |
| `const_fold.js` | Constant folding verification | 19 |
| `dead_code.js` | Dead code elimination | 5 |

## License

MIT
