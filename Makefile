SHELL := /bin/bash
.PHONY: build test clean run test-basic test-loops test-control-flow test-optimizations test-errors
.PHONY: test-fact test-gcd test-ack test-comments test-negative test-logical
.PHONY: test-for-basic test-for-nested test-for-empty-init test-for-empty-incr test-for-factorial test-for-countdown
.PHONY: test-break-while test-break-for test-continue-while test-continue-for test-break-nested test-continue-nested
.PHONY: test-const test-fold test-dead test-tail
.PHONY: test-const-error test-undefined-var test-undefined-func test-break-outside test-continue-outside test-assign-undefined test-missing-semi test-unexpected-token test-unterminated-comment test-missing-brace test-missing-paren

COMPILER = ./target/release/compiler

build:
	cargo build --release

run: build
	@$(COMPILER) $(FILE)

# Run all tests
test: test-basic test-loops test-control-flow test-optimizations test-errors
	@echo ""
	@echo "========================================="
	@echo "=== ALL TESTS PASSED ==="
	@echo "========================================="

# Test categories
test-basic: build test-fact test-gcd test-ack test-comments test-negative test-logical

test-loops: build test-for-basic test-for-nested test-for-empty-init test-for-empty-incr test-for-factorial test-for-countdown

test-control-flow: build test-break-while test-break-for test-continue-while test-continue-for test-break-nested test-continue-nested

test-optimizations: build test-fold test-dead test-tail

test-errors: build test-const-error test-undefined-var test-undefined-func test-break-outside test-continue-outside test-assign-undefined test-missing-semi test-unexpected-token test-unterminated-comment test-missing-brace test-missing-paren

# Basic feature tests
test-fact: build
	@echo "=== Testing Factorial ==="
	@$(COMPILER) tests/basic/factorial.js > tests/basic/factorial.wat
	@result=$$(wasmtime tests/basic/factorial.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "120" ]; then echo "PASS (got 120)"; else echo "FAIL (expected 120, got $$result)"; exit 1; fi

test-gcd: build
	@echo "=== Testing GCD ==="
	@$(COMPILER) tests/basic/gcd.js > tests/basic/gcd.wat
	@result=$$(wasmtime tests/basic/gcd.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "6" ]; then echo "PASS (got 6)"; else echo "FAIL (expected 6, got $$result)"; exit 1; fi

test-ack: build
	@echo "=== Testing Ackermann ==="
	@$(COMPILER) tests/basic/ackermann.js > tests/basic/ackermann.wat
	@result=$$(wasmtime --wasm tail-call tests/basic/ackermann.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "125" ]; then echo "PASS (got 125)"; else echo "FAIL (expected 125, got $$result)"; exit 1; fi

test-comments: build
	@echo "=== Testing Comments ==="
	@$(COMPILER) tests/basic/comments.js > tests/basic/comments.wat
	@result=$$(wasmtime tests/basic/comments.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "15" ]; then echo "PASS (got 15)"; else echo "FAIL (expected 15, got $$result)"; exit 1; fi

test-negative: build
	@echo "=== Testing Negative Number Literals ==="
	@$(COMPILER) tests/basic/negative.js > tests/basic/negative.wat
	@result=$$(wasmtime tests/basic/negative.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "10" ]; then \
		if grep -q "i32.const -5" tests/basic/negative.wat; then \
			echo "PASS (got 10, negative literals folded)"; \
		else \
			echo "FAIL (got 10, but negative literals not folded)"; exit 1; \
		fi \
	else \
		echo "FAIL (expected 10, got $$result)"; exit 1; \
	fi

test-logical: build
	@echo "=== Testing Logical AND/OR Operators ==="
	@$(COMPILER) tests/basic/logical.js > tests/basic/logical.wat
	@result=$$(wasmtime tests/basic/logical.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "21" ]; then echo "PASS (got 21)"; else echo "FAIL (expected 21, got $$result)"; exit 1; fi

# Loop tests
test-for-basic: build
	@echo "=== Testing For Loop Basic ==="
	@$(COMPILER) tests/loops/for_loop_basic.js > tests/loops/for_loop_basic.wat
	@result=$$(wasmtime tests/loops/for_loop_basic.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "55" ]; then echo "PASS (got 55)"; else echo "FAIL (expected 55, got $$result)"; exit 1; fi

test-for-nested: build
	@echo "=== Testing For Loop Nested ==="
	@$(COMPILER) tests/loops/for_loop_nested.js > tests/loops/for_loop_nested.wat
	@result=$$(wasmtime tests/loops/for_loop_nested.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "225" ]; then echo "PASS (got 225)"; else echo "FAIL (expected 225, got $$result)"; exit 1; fi

test-for-empty-init: build
	@echo "=== Testing For Loop Empty Init ==="
	@$(COMPILER) tests/loops/for_loop_empty_init.js > tests/loops/for_loop_empty_init.wat
	@result=$$(wasmtime tests/loops/for_loop_empty_init.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "10" ]; then echo "PASS (got 10)"; else echo "FAIL (expected 10, got $$result)"; exit 1; fi

test-for-empty-incr: build
	@echo "=== Testing For Loop Empty Increment ==="
	@$(COMPILER) tests/loops/for_loop_empty_incr.js > tests/loops/for_loop_empty_incr.wat
	@result=$$(wasmtime tests/loops/for_loop_empty_incr.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "10" ]; then echo "PASS (got 10)"; else echo "FAIL (expected 10, got $$result)"; exit 1; fi

test-for-factorial: build
	@echo "=== Testing For Loop Factorial ==="
	@$(COMPILER) tests/loops/for_loop_factorial.js > tests/loops/for_loop_factorial.wat
	@result=$$(wasmtime tests/loops/for_loop_factorial.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "720" ]; then echo "PASS (got 720)"; else echo "FAIL (expected 720, got $$result)"; exit 1; fi

test-for-countdown: build
	@echo "=== Testing For Loop Countdown ==="
	@$(COMPILER) tests/loops/for_loop_countdown.js > tests/loops/for_loop_countdown.wat
	@result=$$(wasmtime tests/loops/for_loop_countdown.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "55" ]; then echo "PASS (got 55)"; else echo "FAIL (expected 55, got $$result)"; exit 1; fi

# Control flow tests
test-break-while: build
	@echo "=== Testing Break in While Loop ==="
	@$(COMPILER) tests/control-flow/break_while.js > tests/control-flow/break_while.wat
	@result=$$(wasmtime tests/control-flow/break_while.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "7" ]; then echo "PASS (got 7)"; else echo "FAIL (expected 7, got $$result)"; exit 1; fi

test-break-for: build
	@echo "=== Testing Break in For Loop ==="
	@$(COMPILER) tests/control-flow/break_for.js > tests/control-flow/break_for.wat
	@result=$$(wasmtime tests/control-flow/break_for.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "55" ]; then echo "PASS (got 55)"; else echo "FAIL (expected 55, got $$result)"; exit 1; fi

test-continue-while: build
	@echo "=== Testing Continue in While Loop ==="
	@$(COMPILER) tests/control-flow/continue_while.js > tests/control-flow/continue_while.wat
	@result=$$(wasmtime tests/control-flow/continue_while.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "25" ]; then echo "PASS (got 25)"; else echo "FAIL (expected 25, got $$result)"; exit 1; fi

test-continue-for: build
	@echo "=== Testing Continue in For Loop ==="
	@$(COMPILER) tests/control-flow/continue_for.js > tests/control-flow/continue_for.wat
	@result=$$(wasmtime tests/control-flow/continue_for.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "37" ]; then echo "PASS (got 37)"; else echo "FAIL (expected 37, got $$result)"; exit 1; fi

test-break-nested: build
	@echo "=== Testing Break in Nested Loops ==="
	@$(COMPILER) tests/control-flow/break_nested.js > tests/control-flow/break_nested.wat
	@result=$$(wasmtime tests/control-flow/break_nested.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "25" ]; then echo "PASS (got 25)"; else echo "FAIL (expected 25, got $$result)"; exit 1; fi

test-continue-nested: build
	@echo "=== Testing Continue in Nested Loops ==="
	@$(COMPILER) tests/control-flow/continue_nested.js > tests/control-flow/continue_nested.wat
	@result=$$(wasmtime tests/control-flow/continue_nested.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "12" ]; then echo "PASS (got 12)"; else echo "FAIL (expected 12, got $$result)"; exit 1; fi

# Optimization tests
test-fold: build
	@echo "=== Testing Constant Folding ==="
	@$(COMPILER) tests/optimizations/const_fold.js > tests/optimizations/const_fold.wat
	@result=$$(wasmtime tests/optimizations/const_fold.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "19" ]; then \
		if grep -q "i32.const 11" tests/optimizations/const_fold.wat && grep -q "i32.const 8" tests/optimizations/const_fold.wat; then \
			echo "PASS (got 19, constants folded)"; \
		else \
			echo "FAIL (got 19, but constants not folded)"; exit 1; \
		fi \
	else \
		echo "FAIL (expected 19, got $$result)"; exit 1; \
	fi

test-dead: build
	@echo "=== Testing Dead Code Elimination ==="
	@$(COMPILER) tests/optimizations/dead_code.js > tests/optimizations/dead_code.wat
	@result=$$(wasmtime tests/optimizations/dead_code.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "5" ]; then \
		if ! grep -q "local \$$y" tests/optimizations/dead_code.wat && ! grep -q "local \$$z" tests/optimizations/dead_code.wat; then \
			echo "PASS (got 5, dead code eliminated)"; \
		else \
			echo "FAIL (got 5, but dead code not eliminated)"; exit 1; \
		fi \
	else \
		echo "FAIL (expected 5, got $$result)"; exit 1; \
	fi

test-tail: build
	@echo "=== Testing Tail Call Elimination ==="
	@$(COMPILER) tests/basic/ackermann.js > tests/basic/ackermann.wat
	@if grep -q "return_call" tests/basic/ackermann.wat; then \
		result=$$(wasmtime --wasm tail-call tests/basic/ackermann.wat --invoke _start 2>&1 | tail -1); \
		if [ "$$result" = "125" ]; then \
			echo "PASS (got 125, tail calls optimized)"; \
		else \
			echo "FAIL (expected 125, got $$result)"; exit 1; \
		fi \
	else \
		echo "FAIL (no return_call instructions found)"; exit 1; \
	fi

# Error tests
test-const-error: build
	@echo "=== Testing Const Reassignment Error ==="
	@output=$$($(COMPILER) tests/errors/const_error.js 2>&1 || true); \
	if echo "$$output" | grep -q "Cannot reassign const"; then \
		echo "PASS (const reassignment error detected)"; \
	else \
		echo "FAIL (const reassignment not detected)"; \
		echo "$$output"; \
		exit 1; \
	fi

test-undefined-var: build
	@echo "=== Testing Undefined Variable Error ==="
	@output=$$($(COMPILER) tests/errors/undefined_variable.js 2>&1 || true); \
	if echo "$$output" | grep -q "Undefined variable"; then \
		echo "PASS (undefined variable error detected)"; \
	else \
		echo "FAIL (undefined variable not detected)"; \
		echo "$$output"; \
		exit 1; \
	fi

test-undefined-func: build
	@echo "=== Testing Undefined Function Error ==="
	@output=$$($(COMPILER) tests/errors/undefined_function.js 2>&1 || true); \
	if echo "$$output" | grep -q "Undefined function"; then \
		echo "PASS (undefined function error detected)"; \
	else \
		echo "FAIL (undefined function not detected)"; \
		echo "$$output"; \
		exit 1; \
	fi

test-break-outside: build
	@echo "=== Testing Break Outside Loop Error ==="
	@output=$$($(COMPILER) tests/errors/break_outside_loop.js 2>&1 || true); \
	if echo "$$output" | grep -q "Break statement outside"; then \
		echo "PASS (break outside loop error detected)"; \
	else \
		echo "FAIL (break outside loop not detected)"; \
		echo "$$output"; \
		exit 1; \
	fi

test-continue-outside: build
	@echo "=== Testing Continue Outside Loop Error ==="
	@output=$$($(COMPILER) tests/errors/continue_outside_loop.js 2>&1 || true); \
	if echo "$$output" | grep -q "Continue statement outside"; then \
		echo "PASS (continue outside loop error detected)"; \
	else \
		echo "FAIL (continue outside loop not detected)"; \
		echo "$$output"; \
		exit 1; \
	fi

test-assign-undefined: build
	@echo "=== Testing Assign to Undefined Variable Error ==="
	@output=$$($(COMPILER) tests/errors/assign_undefined.js 2>&1 || true); \
	if echo "$$output" | grep -q "Undefined variable"; then \
		echo "PASS (assign to undefined error detected)"; \
	else \
		echo "FAIL (assign to undefined not detected)"; \
		echo "$$output"; \
		exit 1; \
	fi

test-missing-semi: build
	@echo "=== Testing Missing Semicolon Error ==="
	@output=$$($(COMPILER) tests/errors/missing_semicolon.js 2>&1 || true); \
	if echo "$$output" | grep -q "Expected.*Semicolon"; then \
		echo "PASS (missing semicolon error detected)"; \
	else \
		echo "FAIL (missing semicolon not detected)"; \
		echo "$$output"; \
		exit 1; \
	fi

test-unexpected-token: build
	@echo "=== Testing Unexpected Token Error ==="
	@output=$$($(COMPILER) tests/errors/unexpected_token.js 2>&1 || true); \
	if echo "$$output" | grep -q "Unexpected"; then \
		echo "PASS (unexpected token error detected)"; \
	else \
		echo "FAIL (unexpected token not detected)"; \
		echo "$$output"; \
		exit 1; \
	fi

test-unterminated-comment: build
	@echo "=== Testing Unterminated Comment Error ==="
	@output=$$($(COMPILER) tests/errors/unterminated_comment.js 2>&1 || true); \
	if echo "$$output" | grep -q "Unterminated"; then \
		echo "PASS (unterminated comment error detected)"; \
	else \
		echo "FAIL (unterminated comment not detected)"; \
		echo "$$output"; \
		exit 1; \
	fi

test-missing-brace: build
	@echo "=== Testing Missing Brace Error ==="
	@output=$$($(COMPILER) tests/errors/missing_brace.js 2>&1 || true); \
	if echo "$$output" | grep -q "Expected.*RBrace"; then \
		echo "PASS (missing brace error detected)"; \
	else \
		echo "FAIL (missing brace not detected)"; \
		echo "$$output"; \
		exit 1; \
	fi

test-const: test-const-error

clean:
	cargo clean
	rm -f tests/*.wat tests/*/*.wat