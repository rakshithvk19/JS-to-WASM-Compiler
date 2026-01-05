# JS to WASM Compiler

A mini JavaScript to WebAssembly Text Format (.wat) compiler written in Rust. Zero dependencies.

## Supported Language Subset

### Types
- **i32**: 32-bit signed integers
- **f32**: 32-bit floating point numbers (IEEE 754 single-precision)

### Operations
- Arithmetic: `+ - * / %` (modulo only for i32)
- Comparisons: `== != < > <= >=`
- Logical: `&&` `||` (with short-circuit evaluation)
- Unary: `-` (negation), `!` (logical NOT)

### Language Features
- Variable declarations: `let` (mutable), `const` (immutable)
- Control flow: `if/else`, `while`, `for`, `break`, `continue`
- Functions with typed parameters and return values
- Block statements `{ ... }`
- Comments: single-line (`//`) and multi-line (`/* */`)

### Type System
- Automatic type inference from literals
- Type widening: i32 → f32 (never narrows f32 → i32)
- First-call wins: Function parameter types locked on first call
- Return type inference based on parameter types
- Strict type checking on assignment (no implicit conversion)

<details>
<summary>Click to expand type system details</summary>

**Literal Syntax:**
- Integer: `5`, `-10`, `1000` → i32
- Float: `3.14`, `3.`, `.5`, `1e10`, `3.14e-5` → f32

**Type Coercion:**
- Binary operations with mixed types → result is f32
- Example: `5 + 3.14` → converts 5 to 5.0, result is 8.14 (f32)
- Assignment: types must match exactly (no implicit conversion)

**Function Types:**
- Parameter types: Set on first call
- Return type: Inferred from return statement (depends on param types)
- Example: `function mul(a, b) { return a * b; }`
  - `mul(5, 3)` → params (i32, i32), returns i32
  - `mul(2.5, 4.0)` → params (f32, f32), returns f32

</details>

## Requirements

- Rust (cargo)
- [wasmtime](https://wasmtime.dev/) for running the output

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

# For tail-call optimized code
wasmtime --wasm tail-call output.wat --invoke _start
```

## Testing

Tests are organized by category and can be run individually or in groups.

```bash
# Run all tests (57 tests total)
make test

# Run by category
make test-basic           # Basic feature tests (6 tests)
make test-loops           # Loop tests (6 tests)
make test-control-flow    # Break/continue tests (6 tests)
make test-optimizations   # Optimization tests (3 tests)
make test-f32             # F32 floating point tests (21 tests)
make test-errors          # Error handling tests (15 tests)
```

<details>
<summary>Click to expand individual test commands</summary>

### Basic Features
```bash
make test-fact        # Factorial (iterative)
make test-gcd         # GCD (Euclidean algorithm)
make test-ack         # Ackermann function (recursive, tail-call)
make test-comments    # Comment parsing
make test-negative    # Negative number literals
make test-logical     # Logical AND/OR operators
```

### Loops
```bash
make test-for-basic       # For loop - sum 1 to 10
make test-for-nested      # Nested for loops
make test-for-empty-init  # For loop with empty init
make test-for-empty-incr  # For loop with empty increment
make test-for-factorial   # Factorial using for loop
make test-for-countdown   # For loop counting down
```

### Control Flow
```bash
make test-break-while       # Break in while loop
make test-break-for         # Break in for loop
make test-continue-while    # Continue in while loop
make test-continue-for      # Continue in for loop
make test-break-nested      # Break in nested loops
make test-continue-nested   # Continue in nested loops
```

### Optimizations
```bash
make test-fold    # Constant folding
make test-dead    # Dead code elimination
make test-tail    # Tail call optimization
```

### F32 Tests
```bash
make test-f32-basic-arith     # Basic f32 arithmetic
make test-f32-literals        # All f32 literal formats
make test-f32-division        # F32 vs i32 division
make test-f32-negative        # Negative f32 values
make test-f32-comparisons     # F32 comparisons
make test-f32-mixed-arith     # Mixed type arithmetic
make test-f32-mixed-comp      # Mixed type comparisons
make test-f32-mixed-logical   # Mixed type logical ops
make test-f32-var-inference   # Variable type inference
make test-f32-const-vars      # Const with f32
make test-f32-func-return     # Function returning f32
make test-f32-func-params     # F32 function parameters
make test-f32-func-mixed      # Mixed type parameters
make test-f32-if-cond         # F32 in if conditions
make test-f32-while           # F32 in while loops
make test-f32-for             # F32 in for loops
make test-f32-const-fold      # F32 constant folding
make test-f32-dead            # F32 dead code elimination
make test-f32-zero-comp       # Zero comparison edge cases
make test-f32-truthiness      # F32 truthiness behavior
make test-f32-chained         # Complex chained operations
```

### Error Tests
```bash
make test-const-error         # Const reassignment
make test-undefined-var       # Undefined variable
make test-undefined-func      # Undefined function
make test-break-outside       # Break outside loop
make test-continue-outside    # Continue outside loop
make test-assign-undefined    # Assign to undefined
make test-missing-semi        # Missing semicolon
make test-unexpected-token    # Unexpected token
make test-unterminated-comment # Unterminated comment
make test-missing-brace       # Missing brace
make test-missing-paren       # Missing paren
make test-f32-type-mismatch   # F32 type mismatch
make test-f32-modulo-error    # F32 modulo error
make test-f32-inconsistent-return  # Inconsistent return types
make test-f32-param-mismatch  # Function param mismatch
```

</details>

## Architecture

![Architecture Diagram](architecture.png)

<details>
<summary>Click to expand architecture details</summary>

```
src/
├── main.rs       # CLI entry point
├── error.rs      # Error types and handling
├── lexer.rs      # Tokenization (supports i32 and f32 literals)
├── ast.rs        # AST node definitions with Type enum
├── parser.rs     # Recursive descent parser
├── semantic.rs   # Type inference, validation, stores types in AST
├── optimizer.rs  # Constant folding & dead code elimination
└── codegen.rs    # WAT code generation with type-aware instructions
```

### Pipeline

```
JS Source → Lexer → Tokens → Parser → AST → Semantic Analyzer → Optimizer → CodeGen → WAT
                                              (2-pass analysis)
```

**Semantic Analysis (Two-Pass):**
1. First pass: Analyze functions with default i32 parameters
2. Analyze top-level: First function calls set actual parameter types
3. Second pass: Re-analyze functions with correct parameter types to finalize return types

This ensures return types correctly reflect parameter types (e.g., `function mul(a, b) { return a * b; }` returns i32 for integer params, f32 for float params).

</details>

## F32 Floating Point Support

The compiler supports 32-bit floating point numbers with automatic type inference and conversion.

<details>
<summary>Click to expand F32 features</summary>

### Literal Formats
```javascript
let a = 3.14;      // Decimal point
let b = 3.;        // Trailing dot
let c = .5;        // Leading dot
let d = 1e10;      // Scientific notation
let e = 3.14e-5;   // Scientific with exponent
```

### Type Inference
```javascript
let x = 5;         // x: i32
let y = 3.14;      // y: f32
let z = x + y;     // z: f32 (auto-widens i32 to f32)
```

### Automatic Type Conversion
```javascript
5 + 3.14    // i32 + f32 → 8.14 (f32)
5 / 2       // i32 / i32 → 2 (integer division)
5.0 / 2.0   // f32 / f32 → 2.5 (float division)
```

### Mixed Type Operations
```javascript
let a = 5 && 3.14;     // Returns 3.14 (f32)
let b = 0.0 || 5;      // Returns 5.0 (f32, converted)
let c = 5 == 5.0;      // Returns 1 (i32, comparison result)
```

### Function Type Inference
```javascript
function multiply(a, b) {
  return a * b;
}

multiply(2.5, 4.0);   // First call: params (f32, f32), returns f32
multiply(3.0, 5.0);   // OK: matches first call
multiply(5, 3);       // ERROR: type mismatch
```

### Restrictions
- **No f32 modulo:** `5.0 % 3.0` → Compile error
- **No implicit narrowing:** Cannot assign f32 to i32 variable
- **Precision limits:** ~7 decimal digits, may have rounding errors

### Truthiness
```javascript
if (0.0) { }      // Falsy (not executed)
if (3.14) { }     // Truthy (executed)
if (-0.5) { }     // Truthy (any non-zero)
```

</details>

## Optimizations

<details>
<summary>Click to expand optimization details</summary>

### Constant Folding
Evaluates constant expressions at compile time (works for both i32 and f32).

```javascript
let x = 3 + 4 * 2;     // Compiled as: i32.const 11
let y = 3.0 + 4.0;     // Compiled as: f32.const 7.0
```

### Negative Number Folding
Folds unary negation of constants into single negative literals.

```javascript
let x = -5;            // Compiled as: i32.const -5
let y = -3.14;         // Compiled as: f32.const -3.14
let z = 10 + -3;       // Compiled as: i32.const 7
```

### Dead Code Elimination
Removes unreachable code after `return` statements and eliminates constant-false conditions.

```javascript
function test() {
  return 5;
  let x = 10;     // Eliminated - unreachable
}

if (0) { }        // Eliminated - condition always false (i32)
if (0.0) { }      // Eliminated - condition always false (f32)
while (0.0) { }   // Eliminated - loop never executes
```

### Tail Call Elimination
Optimizes recursive calls in tail position using `return_call` instruction.

```javascript
function ack(m, n) {
  if (n == 0) return ack(m - 1, 1);  // Uses return_call
  ...
}
```

Run with: `wasmtime --wasm tail-call output.wat --invoke _start`

### Const Immutability
Enforces `const` variables cannot be reassigned.

```javascript
const x = 10;
x = 20;  // Compiler error: Cannot reassign const variable 'x'
```

</details>

## Type System

<details>
<summary>Click to expand type system rules</summary>

### Type Inference
```javascript
let x = 5;         // x: i32 (inferred from literal)
let y = 3.14;      // y: f32 (inferred from literal)
let z = x + y;     // z: f32 (inferred from operation)
```

### Type Widening (Auto-Conversion)
**Rule:** Always widen i32 → f32, never narrow f32 → i32

```javascript
5 + 3.14    // i32 + f32 → 5.0 + 3.14 = 8.14 (f32)
5 == 5.0    // i32 == f32 → 5.0 == 5.0 → true (converts for comparison)
```

### Assignment Rules
**No implicit conversion on assignment** - types must match exactly:

```javascript
let x = 5;         // x: i32
x = 10;            // ✅ OK: i32 = i32
x = 3.14;          // ❌ ERROR: Cannot assign f32 to i32

let y = 3.14;      // y: f32
y = 2.5;           // ✅ OK: f32 = f32
y = 5;             // ❌ ERROR: Cannot assign i32 to f32
```

### Division Behavior
```javascript
5 / 2       // i32 / i32 → 2 (truncates)
5.0 / 2.0   // f32 / f32 → 2.5 (float division)
5 / 2.0     // i32 / f32 → 2.5 (auto-converts to f32)
```

### Logical Operators
Supports `&&` (AND) and `||` (OR) with short-circuit evaluation and type widening:

```javascript
5 && 3        // i32 && i32 → 3 (i32)
5.0 && 3.0    // f32 && f32 → 3.0 (f32)
5 && 3.0      // i32 && f32 → 3.0 (f32, both converted)
0.0 || 5      // f32 || i32 → 5.0 (f32, converted)
```

### Comparison Results
All comparisons return i32 (0 for false, 1 for true):

```javascript
let x = 5 < 3.14;   // x: i32 (value is 0)
let y = 5.0 == 5;   // y: i32 (value is 1)
```

### Function Signatures
**First-Call Wins:** Parameter types locked on first call, return type depends on parameters:

```javascript
function add(a, b) { return a + b; }

add(5, 3);          // First call: params (i32, i32), returns i32
add(10, 20);        // ✅ OK: matches (i32, i32)
add(5.0, 3.0);      // ❌ ERROR: Expected (i32, i32), got (f32, f32)
```

If first call was `add(5.0, 3.0)`, params would be (f32, f32) and return f32.

</details>

## Loop Control Flow

<details>
<summary>Click to expand loop control details</summary>

### Break Statement
Exits the current loop immediately.

```javascript
for (let i = 1; i <= 100; i = i + 1) {
  if (i % 7 == 0) {
    result = i;  // Found first multiple of 7
    break;       // Exit loop
  }
}
```

### Continue Statement
Skips the rest of the current iteration and proceeds to the next.

```javascript
for (let i = 1; i <= 10; i = i + 1) {
  if (i % 3 == 0) {
    continue;  // Skip multiples of 3
  }
  sum = sum + i;
}
```

**Note:** In `for` loops, `continue` properly executes the increment before the next iteration.

### For Loop Scoping
Variables declared in for loop init are scoped to the loop:

```javascript
for (let i = 0; i < 5; i = i + 1) {
  // i is accessible here
}
// i is NOT accessible here
```

</details>

## Error Handling

Compiler provides detailed error messages with line numbers.

<details>
<summary>Click to expand error categories</summary>

### Lexer Errors
- Unexpected characters
- Unterminated block comments
- Invalid number literals

### Parser Errors  
- Missing semicolons, braces, parentheses
- Unexpected tokens
- Invalid syntax
- Missing closing braces (detected early with clear error messages)

### Semantic Errors
- Undefined variables or functions
- Const variable reassignment
- Break/Continue outside of loops
- Type mismatch on assignment
- F32 modulo operation
- Inconsistent function return types
- Function parameter type mismatch on subsequent calls

**Example error output:**
```
Semantic Error at line 5: Cannot reassign const variable 'x'
Semantic Error at line 7: Type mismatch: cannot assign F32 to I32 variable 'y'
Semantic Error at line 3: Modulo operation not supported for f32 types
```

</details>

## Test Organization

Tests are organized in categorized folders:

<details>
<summary>Click to expand test structure</summary>

```
tests/
├── basic/          # Core language features (6 tests)
│   ├── factorial.js
│   ├── gcd.js
│   ├── ackermann.js
│   ├── comments.js
│   ├── negative.js
│   └── logical.js
├── loops/          # For loop variations (6 tests)
│   ├── for_loop_basic.js
│   ├── for_loop_nested.js
│   ├── for_loop_empty_init.js
│   ├── for_loop_empty_incr.js
│   ├── for_loop_factorial.js
│   └── for_loop_countdown.js
├── control-flow/   # Break/Continue statements (6 tests)
│   ├── break_while.js
│   ├── break_for.js
│   ├── continue_while.js
│   ├── continue_for.js
│   ├── break_nested.js
│   └── continue_nested.js
├── f32/            # F32 floating point tests (21 tests)
│   ├── Basic operations (6)
│   ├── Mixed types (3)
│   ├── Type inference (2)
│   ├── Functions (3)
│   ├── Control flow (3)
│   ├── Optimizations (2)
│   └── Edge cases (3)
├── optimizations/  # Optimization verifications (2 tests)
│   ├── const_fold.js
│   └── dead_code.js
└── errors/         # Error handling tests (15 tests)
    ├── const_error.js
    ├── undefined_variable.js
    ├── undefined_function.js
    ├── break_outside_loop.js
    ├── continue_outside_loop.js
    ├── assign_undefined.js
    ├── missing_semicolon.js
    ├── unexpected_token.js
    ├── unterminated_comment.js
    ├── missing_brace.js
    ├── missing_paren.js
    ├── f32_type_mismatch_assignment.js
    ├── f32_modulo_error.js
    ├── f32_inconsistent_return.js
    └── f32_function_param_mismatch.js
```

**Total Tests: 57**
- Happy path: 42 tests
- Error cases: 15 tests

</details>

## Source Location Comments

Generated WAT includes comments mapping instructions to original JS line numbers for debugging.

<details>
<summary>Click to expand example</summary>

```wat
;; line 3
i32.const 5
local.set $x
;; line 4
f32.const 3.14
local.set $y
```

</details>

## Test Programs

<details>
<summary>Click to expand test suite details</summary>

### Basic Features
| Test | Description | Expected |
|------|-------------|----------|
| `factorial.js` | Iterative factorial (5!) | 120 |
| `gcd.js` | Euclidean GCD(48, 18) | 6 |
| `ackermann.js` | Recursive Ackermann(3,4) | 125 |
| `comments.js` | Comment parsing | 15 |
| `negative.js` | Negative number literals | 10 |
| `logical.js` | Logical AND/OR operators | 21 |

### Loops
| Test | Description | Expected |
|------|-------------|----------|
| `for_loop_basic.js` | Sum 1 to 10 | 55 |
| `for_loop_nested.js` | 5x5 multiplication table | 225 |
| `for_loop_empty_init.js` | For loop with empty init | 10 |
| `for_loop_empty_incr.js` | For loop with empty increment | 10 |
| `for_loop_factorial.js` | Factorial using for loop (6!) | 720 |
| `for_loop_countdown.js` | Countdown from 10 to 1 | 55 |

### Control Flow
| Test | Description | Expected |
|------|-------------|----------|
| `break_while.js` | Break in while - find first ÷7 | 7 |
| `break_for.js` | Break in for - sum until > 50 | 55 |
| `continue_while.js` | Continue in while - sum odds | 25 |
| `continue_for.js` | Continue in for - skip ÷3 | 37 |
| `break_nested.js` | Break nested - pair sum to 7 | 25 |
| `continue_nested.js` | Continue nested - skip evens | 12 |

### F32 Tests (21 tests)
| Category | Count | Examples |
|----------|-------|----------|
| Basic operations | 6 | Arithmetic, literals, division, negation, comparisons |
| Mixed types | 3 | Mixed arithmetic, comparisons, logical operators |
| Type inference | 2 | Variable inference, const variables |
| Functions | 3 | Return types, parameters, mixed params |
| Control flow | 3 | If conditions, while loops, for loops |
| Optimizations | 2 | Constant folding, dead code elimination |
| Edge cases | 3 | Zero comparisons, truthiness, chained operations |

### Optimizations
| Test | Description | Expected |
|------|-------------|----------|
| `const_fold.js` | Constant folding verification | 19 |
| `dead_code.js` | Dead code elimination | 5 |

### Error Tests (15 tests)
All error tests verify that the compiler correctly detects and reports errors with appropriate messages.

</details>

## F32 Design Decisions

<details>
<summary>Click to expand F32 design rationale</summary>

### Why F32 (not F64)?
- **Sufficient precision:** 7 decimal digits is enough for this educational project
- **Simpler type system:** Only one float type means simpler conversion rules
- **Single conversion path:** Only i32 ↔ f32 (not i32 ↔ f32, i32 ↔ f64, f32 ↔ f64)
- **Cleaner implementation:** Fewer edge cases and type combinations

### Type Coercion Philosophy
- **Always widen, never narrow:** i32 → f32 is automatic, f32 → i32 requires explicit handling
- **Preserve precision:** Once a value becomes f32, it stays f32
- **Type safety:** Assignment requires exact type match, preventing accidental precision loss

### First-Call Wins Rationale
Since JavaScript is dynamically typed, we defer type decisions to runtime:
- Function signature is determined by actual usage
- First call establishes the contract
- Subsequent calls must honor that contract
- Allows polymorphic-like behavior without explicit generics

### Modulo Restriction
WASM has no `f32.rem` instruction, and implementing it with `a - (b * floor(a / b))` adds complexity. Since modulo on floats is rare, we simply disallow it.

### Testing Strategy
F32 tests use approximate matching (e.g., `grep -q "16.42"`) rather than exact equality due to floating-point precision limitations. This is a common practice in floating-point testing.

</details>

## Future Improvements

<details>
<summary>Click to expand roadmap</summary>

### Language Features
- [x] For loops
- [x] Break/Continue statements
- [x] Better error messages with line numbers
- [x] Floating point numbers (f32)
- [ ] Arrays
- [ ] Strings
- [ ] Objects/Structs 
- [ ] First-class functions
- [ ] Closures

### Architectural Improvements
- [ ] Implement Visitor pattern for AST traversal (reduces code duplication)
- [ ] Introduce proper IR (Intermediate Representation) for better optimization
- [ ] Type inference with Hindley-Milner algorithm
- [ ] Better error recovery in parser

### Fullstack Conversion
- [ ] Axum REST API for compilation service
- [ ] Browser WebAssembly execution engine
- [ ] Web Playground UI with Monaco editor
- [ ] Real-time compilation and execution
- [ ] Syntax highlighting and error markers

</details>

## Documentation

- **[F32 Design Document](docs/f32-design.md)** - Complete design decisions for f32 support
- **[F32 Test Cases](docs/f32%20testcases.md)** - F32 test examples
- **[Architecture Diagram](architecture.png)** - Visual overview

## License

MIT

---

**Project Status:** ✅ Production Ready  
**Test Coverage:** 57 tests (100% passing)  
**Language Support:** JavaScript subset with i32 and f32 types  
**Zero Dependencies:** Pure Rust implementation
