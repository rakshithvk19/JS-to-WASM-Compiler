# F32 Floating Point Design Decisions

## Overview
This document outlines all design decisions for adding 32-bit floating point number support to the JS-to-WASM compiler.

## Type System

### Supported Types
- **i32**: 32-bit signed integers (existing)
- **f32**: 32-bit floating point numbers (new)

### Type Precision
- **f32**: ~7 decimal digits of precision
- Range: ±3.4 × 10^38
- Uses IEEE 754 single-precision format

**Note on Testing:** Due to f32 precision limitations, tests should use approximate matching rather than exact equality. For example, `-1.64` may be represented as `-1.6400001` in actual execution.

---

## Literal Syntax

### Integer Literals (i32)
```javascript
5
-10
0
1000
```
**Rule:** No decimal point, no exponent → i32

### Float Literals (f32)
```javascript
3.14      // decimal point
3.        // trailing dot
.14       // leading dot
1e10      // scientific notation (1 × 10^10)
3.14e-5   // scientific with negative exponent
```
**Rule:** Has `.` or `e` → f32

**Lexer Implementation Note:** For trailing dots (e.g., `3.`), the lexer consumes the dot only if it's NOT followed by an alphabetic character or underscore. This prevents parsing errors like `3.foo` where the dot might be intended as a property accessor (though not supported in this compiler).

---

## Type Inference

### Variable Types
Variables infer their type from initialization:

```javascript
let x = 5;      // x: i32
let y = 3.14;   // y: f32
const z = 1e10; // z: f32
```

### Fixed Type Variables
Once a variable has a type, it cannot change:

```javascript
let x = 5;    // x: i32
x = 10;       // ✅ OK: i32 = i32
x = 3.14;     // ❌ ERROR: Cannot assign f32 to i32 variable
```

**Implementation:** Track variable types in semantic analyzer using scoped HashMap stack.

---

## Type Coercion (Auto-Conversion)

### Implicit Conversions
**Rule:** Always widen i32 → f32, never narrow f32 → i32

### Binary Operations
```javascript
5 + 3.14    // i32 + f32 → 5.0 + 3.14 = 8.14 (f32)
3.14 + 5    // f32 + i32 → 3.14 + 5.0 = 8.14 (f32)
5 + 3       // i32 + i32 → 8 (i32)
5.0 + 3.0   // f32 + f32 → 8.0 (f32)
```

**Result type:** Wider of the two operand types.

### Comparisons
```javascript
5 == 5.0    // i32 == f32 → 5.0 == 5.0 → true
5 < 3.14    // i32 < f32 → 5.0 < 3.14 → false
```

**Auto-convert to f32 for comparison.**

### WASM Code Generation
```wat
;; For: 5 + 3.14
i32.const 5
f32.convert_i32_s    ;; Convert i32 to f32
f32.const 3.14
f32.add              ;; Add as f32
```

---

## Division Behavior

### Integer Division (i32 / i32)
```javascript
5 / 2      // i32 / i32 → 2 (truncates decimal)
10 / 3     // i32 / i32 → 3
```
Uses: `i32.div_s` (signed integer division)

### Float Division (f32 / f32)
```javascript
5.0 / 2.0  // f32 / f32 → 2.5
10.0 / 3.0 // f32 / f32 → 3.333...
```
Uses: `f32.div`

### Mixed Division (i32 / f32 or f32 / i32)
```javascript
5 / 2.0    // i32 / f32 → 5.0 / 2.0 → 2.5 (f32)
10.0 / 3   // f32 / i32 → 10.0 / 3.0 → 3.333... (f32)
```
**Convert to f32, then divide.**

---

## Function Signatures

### Return Type Inference
Infer return type from the return statement:

```javascript
function add(a, b) {
  return a + b;  // If a + b is f32, function returns f32
}

function getInt() {
  return 5;      // Returns i32
}

function getFloat() {
  return 3.14;   // Returns f32
}
```

**Implementation:** 
- Semantic analyzer analyzes return expression type
- Return type depends on parameter types (which are set on first call)
- Store in AST: `Function.return_type: Option<Type>`
- Codegen reads from AST and generates `(result i32)` or `(result f32)`

**Two-Pass Analysis:**
Since return types depend on parameter types, and parameter types are only known after the first call:
1. **First pass:** Analyze function with default i32 parameters to get preliminary types
2. **Analyze top-level code:** First function calls set actual parameter types
3. **Second pass:** Re-analyze functions with correct parameter types to update return types

This ensures that `function multiply(a, b) { return a * b; }` returns i32 when called with integers, but f32 when called with floats.

### Parameter Type Inference (First-Call Wins)
Infer parameter types from the first function call:

```javascript
function add(a, b) {
  return a + b;
}

add(5, 3);      // First call: a=i32, b=i32 → function signature locked
add(5, 3);      // ✅ OK: matches signature
add(5.0, 3.0);  // ❌ ERROR: Expected i32, got f32
```

**Alternative calls:**
```javascript
add(5.0, 3.0);  // If this was first call: a=f32, b=f32
```

**Implementation:**
- On first call in semantic analyzer, record parameter types in function info
- Store in AST: `Function.param_types: Option<Vec<Type>>`
- Validate all subsequent calls match the first call's types
- Codegen reads from AST and generates typed WASM params: `(param $a i32)` or `(param $a f32)`

**Note:** This means the return type is finalized only AFTER the first call, requiring the two-pass analysis described above.

---

## Type Checking Rules

### Variable Declaration
```javascript
let x = 5;      // x: i32
let y = 3.14;   // y: f32
const z = 1e5;  // z: f32
```

### Assignment
```javascript
let x = 5;      // x: i32
x = 10;         // ✅ OK: i32 = i32
x = 3.14;       // ❌ ERROR: Cannot assign f32 to i32 variable

let y = 3.14;   // y: f32
y = 2.5;        // ✅ OK: f32 = f32
y = 5;          // ❌ ERROR: Cannot assign i32 to f32 variable
```

**Note:** No implicit conversion on assignment - types must match exactly.

### Binary Operations
```javascript
i32 + i32 → i32
f32 + f32 → f32
i32 + f32 → f32 (convert i32 to f32)
f32 + i32 → f32 (convert i32 to f32)
```

**Applies to:** `+`, `-`, `*`, `/`, `%`, `==`, `!=`, `<`, `>`, `<=`, `>=`

### Logical Operations
```javascript
5 && 3        // i32 && i32 → i32 (last value)
5.0 && 3.0    // f32 && f32 → f32 (last value)
5 && 3.0      // i32 && f32 → f32 (convert and return last)
```

**Result:** Type of the returned value (respects short-circuit).

**Implementation Detail:** In mixed-type logical operations, BOTH operands are converted to the wider type (f32):
```wat
;; For: 5 && 3.14
i32.const 5
f32.convert_i32_s         ;; Convert left operand to f32
local.tee $_result        ;; Store as f32
f32.const 0.0
f32.eq                    ;; Check if falsy
if (result f32)           ;; Both branches return f32
  local.get $_result      ;; Return left (already f32)
else
  f32.const 3.14          ;; Return right (f32)
end
```

This ensures type consistency - the result variable `$_result` is always the wider type, and all values stored in it are properly converted.

### Unary Operations
```javascript
-5      // i32 → i32
-3.14   // f32 → f32
!5      // i32 → i32 (0 or 1)
!3.14   // f32 → i32 (0 or 1, truthiness check)
```

**Not (`!`) always returns i32** (boolean result).

---

## Scoping Rules

### For Loop Scoping
Variables declared in for loop initialization are scoped to the loop:

```javascript
for (let i = 0; i < 10; i = i + 1) {
  // i is accessible here
}
// i is NOT accessible here
```

**Implementation:** The semantic analyzer creates a new scope for the entire for loop (including init, condition, increment, and body).

---

## Special Values

### f32 Special Cases
```javascript
1.0 / 0.0     // Infinity
-1.0 / 0.0    // -Infinity
0.0 / 0.0     // NaN (Not a Number)
```

### NaN Behavior
```javascript
let nan = 0.0 / 0.0;
nan == nan;   // false (NaN != NaN always)
nan < 5.0;    // false
nan > 5.0;    // false
```

**Note:** NaN comparisons always return false.

---

## WASM Instructions

### f32 Operations
```wat
f32.const 3.14          ;; Load f32 constant
f32.add                 ;; Addition
f32.sub                 ;; Subtraction
f32.mul                 ;; Multiplication
f32.div                 ;; Division
f32.eq                  ;; Equal
f32.ne                  ;; Not equal
f32.lt                  ;; Less than
f32.gt                  ;; Greater than
f32.le                  ;; Less or equal
f32.ge                  ;; Greater or equal
f32.neg                 ;; Negation
```

### Type Conversions
```wat
;; i32 → f32
i32.const 5
f32.convert_i32_s       ;; 5 (i32) → 5.0 (f32)

;; f32 → i32 (truncate, for explicit casts later)
f32.const 3.14
i32.trunc_f32_s         ;; 3.14 (f32) → 3 (i32)
```

**Note:** f32 → i32 conversion truncates (not rounds). Currently, f32 → i32 is only used internally for truthiness checks, not exposed to users.

---

## Modulo Operation

### Problem
WASM has `i32.rem_s` but **NO `f32.rem`** instruction!

### Solution: Disallow
```javascript
5 % 3      // ✅ OK: i32 % i32
5.0 % 3.0  // ❌ ERROR: Modulo not supported for f32
```

**Reasoning:** 
- Simpler implementation
- Modulo on floats is rare
- Can add later if needed using `a - (b * floor(a / b))`

**Error Handling:** Semantic analyzer detects and rejects f32 modulo operations during type checking.

---

## Edge Cases & Decisions

### Case 1: Multiple Return Types
```javascript
function foo(x) {
  if (x > 0) return 5;      // i32
  return 3.14;              // f32
}
```
**Decision:** ❌ ERROR - all returns must have same type

**Implementation:** Semantic analyzer's `infer_return_type_from_stmts` recursively searches all return statements (including nested in if/while/for/blocks) and validates consistency.

### Case 2: Comparison Result Type
```javascript
let x = 5 < 3.14;  // What type is x?
```
**Decision:** x is i32 (comparisons return 0 or 1)

### Case 3: Truthiness for f32
```javascript
if (3.14) { ... }  // Is 3.14 truthy?
```
**Decision:** 
- 0.0 = falsy
- Any other f32 = truthy
- Same as i32: 0 = falsy, non-zero = truthy

**WASM Implementation:**
```wat
;; For: if (3.14) { ... }
f32.const 3.14
f32.const 0.0
f32.ne              ;; Check if != 0.0, returns i32
if                  ;; Use i32 result for branching
  ...
end
```

### Case 4: Default Function Return
```javascript
function foo() {
  let x = 5;
  // No explicit return
}
```
**Decision:** Returns i32 (default 0) - keep current behavior

---

## Architecture & Implementation Strategy

### Separation of Concerns

**Semantic Analyzer (semantic.rs):**
- Type inference for variables and expressions
- Function signature inference (parameters and return types)
- Type checking and validation
- Stores inferred types in AST

**AST (ast.rs):**
- `Function.param_types: Option<Vec<Type>>` - Set on first call
- `Function.return_type: Option<Type>` - Inferred from body
- Acts as the "contract" between semantic analysis and code generation

**Code Generator (codegen.rs):**
- Reads types from AST (no re-inference)
- Builds `function_return_types` HashMap for quick lookup
- Generates type-correct WASM instructions
- Handles type conversions as needed

**Optimizer (optimizer.rs):**
- Constant folding for both i32 and f32
- Dead code elimination (handles f32 conditions: `if (0.0)` is dead code)
- Type-preserving optimizations

### Two-Pass Semantic Analysis

Due to the "first-call wins" parameter type inference:

1. **First Pass - Function Analysis:**
   - Analyze each function with default i32 parameters
   - Infer preliminary return types
   - Register all functions

2. **Top-Level Analysis:**
   - Analyze top-level code
   - First function calls set actual parameter types
   - Parameter types stored in function info

3. **Second Pass - Re-Analysis:**
   - For functions whose parameter types were set by calls
   - Re-analyze function body with correct parameter types
   - Update return types based on actual parameter types

**Why needed:** A function like `function multiply(a, b) { return a * b; }` returns different types based on parameter types:
- `multiply(5, 3)` → params: (i32, i32), returns: i32
- `multiply(2.5, 4.0)` → params: (f32, f32), returns: f32

The return type DEPENDS on parameter types, which are only known after the first call.

---

## Implementation Order

1. ✅ **Lexer** - Parse f32 literals (including trailing dots and scientific notation)
2. ✅ **AST** - Add `Type` enum, `NumberF32` expression, type fields in `Function`
3. ✅ **Type System** - Type inference and validation in semantic analyzer
4. ✅ **Semantic** - Two-pass analysis, type checking, store types in AST
5. ✅ **Codegen** - Read types from AST, generate f32 instructions, handle conversions
6. ✅ **Optimizer** - Fold f32 constants, dead code elimination with f32
7. ✅ **Testing** - Comprehensive f32 test suite with 26 tests

---

## Error Handling

All type-related errors are caught in the **semantic analysis phase** before code generation:

- **Type mismatch on assignment:** `let x = 5; x = 3.14;` → Semantic Error
- **Modulo on f32:** `5.0 % 3.0` → Semantic Error  
- **Inconsistent return types:** Multiple returns with different types → Semantic Error
- **Function parameter mismatch:** Second call doesn't match first → Semantic Error

Codegen assumes all type checking is complete and focuses on correct WASM generation.

---

## Optimizations

### Constant Folding
Works for both i32 and f32:

```javascript
let x = 3.0 + 4.0;  // Compiled as: f32.const 7.0
let y = 5 + 3;      // Compiled as: i32.const 8
```

**Implementation:** Optimizer pattern-matches on `Expr::Number` and `Expr::NumberF32`, evaluating operations at compile time.

### Dead Code Elimination
Handles f32 conditions:

```javascript
if (0.0) {          // Dead code: 0.0 is falsy
  let x = 10;       // This block is eliminated
}

while (0.0) {       // Dead code: loop never executes
  ...               // Eliminated
}
```

**Implementation:** Optimizer checks for constant `0` (i32) and `0.0` (f32) in conditions and removes unreachable branches.

---

## Why f32 (not f64)?

- **Sufficient precision** - 7 digits is enough for this project
- **Simpler name** - f32 vs f64 
- **Personal preference** - it's cooler!
- **Single float type** - Only one set of conversion rules to manage (i32↔f32), not two (i32↔f32, i32↔f64, f32↔f64)

If we had both f32 AND f64, we'd need rules for:
- i32 → f32
- i32 → f64
- f32 → f64
- f64 → f32
- f32 + f64 = ?

With just f32, only need: i32 ↔ f32

---

## Implementation Challenges Solved

### Challenge 1: Function Return Type Depends on Parameters
**Problem:** `function f(a, b) { return a + b; }` - return type unknown until first call.

**Solution:** Two-pass semantic analysis - analyze with defaults, then re-analyze after first call sets param types.

### Challenge 2: Logical Operators with Mixed Types
**Problem:** `5 && 3.14` - left is i32, right is f32, result should be f32.

**Solution:** 
- Determine result type (wider of the two)
- Convert both operands to result type before storing/returning
- Use type-specific truthiness checks

### Challenge 3: Type Information Flow
**Problem:** Semantic analyzer infers types, codegen needs them.

**Solution:** Store types in AST (`Function.param_types`, `Function.return_type`). Codegen reads from AST and builds lookup maps for efficient access.

### Challenge 4: Trailing Dot Parsing
**Problem:** Distinguish `3.` (float) from `3.foo` (potential property access).

**Solution:** Lexer checks if character after dot is alphanumeric/underscore. Only consume dot if followed by digit, exponent, or non-identifier characters.

### Challenge 5: Floating-Point Precision in Tests
**Problem:** F32 arithmetic may produce values like `-1.6400001` instead of `-1.64`.

**Solution:** Test assertions use pattern matching (grep) rather than exact equality for f32 results.

---

**Document Status:** Complete and Verified  
**Implementation Status:** ✅ Fully Implemented and Tested  
**Last Updated:** January 2026
