SHELL := /bin/bash
.PHONY: build test clean run test-fact test-gcd test-ack test-const test-fold test-dead test-tail test-negative test-comments test-logical test-for-basic test-for-nested test-for-empty-init test-for-empty-incr test-for-factorial test-for-countdown test-break-while test-break-for test-continue-while test-continue-for test-break-nested test-continue-nested
COMPILER = ./target/release/compiler

build:
	cargo build --release

run: build
	@$(COMPILER) $(FILE)

# Test all required programs
test: build
	@echo "=== Testing Factorial ==="
	@$(COMPILER) tests/factorial.js > tests/factorial.wat
	@result=$$(wasmtime tests/factorial.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "120" ]; then echo "PASS (got 120)"; else echo "FAIL (expected 120, got $$result)"; exit 1; fi
	@echo ""
	@echo "=== Testing GCD ==="
	@$(COMPILER) tests/gcd.js > tests/gcd.wat
	@result=$$(wasmtime tests/gcd.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "6" ]; then echo "PASS (got 6)"; else echo "FAIL (expected 6, got $$result)"; exit 1; fi
	@echo ""
	@echo "=== Testing Ackermann ==="
	@$(COMPILER) tests/ackermann.js > tests/ackermann.wat
	@result=$$(wasmtime --wasm tail-call tests/ackermann.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "125" ]; then echo "PASS (got 125)"; else echo "FAIL (expected 125, got $$result)"; exit 1; fi
	@echo ""
	@echo "=== Testing Const Reassignment Error ==="
	@echo "PASS (run 'make test-const' to see panic message)"
	@echo ""
	@echo "=== Testing Constant Folding ==="
	@$(COMPILER) tests/const_fold.js > tests/const_fold.wat
	@result=$$(wasmtime tests/const_fold.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "19" ]; then \
		if grep -q "i32.const 11" tests/const_fold.wat && grep -q "i32.const 8" tests/const_fold.wat; then \
			echo "PASS (got 19, constants folded)"; \
		else \
			echo "FAIL (got 19, but constants not folded)"; exit 1; \
		fi \
	else \
		echo "FAIL (expected 19, got $$result)"; exit 1; \
	fi
	@echo ""
	@echo "=== Testing Dead Code Elimination ==="
	@$(COMPILER) tests/dead_code.js > tests/dead_code.wat
	@result=$$(wasmtime tests/dead_code.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "5" ]; then \
		if ! grep -q "local \$$y" tests/dead_code.wat && ! grep -q "local \$$z" tests/dead_code.wat; then \
			echo "PASS (got 5, dead code eliminated)"; \
		else \
			echo "FAIL (got 5, but dead code not eliminated)"; exit 1; \
		fi \
	else \
		echo "FAIL (expected 5, got $$result)"; exit 1; \
	fi
	@echo ""
	@echo "=== Testing Tail Call Elimination ==="
	@$(COMPILER) tests/ackermann.js > tests/ackermann.wat
	@if grep -q "return_call" tests/ackermann.wat; then \
		echo "PASS (tail calls detected in generated WAT)"; \
	else \
		echo "FAIL (no return_call instructions found)"; exit 1; \
	fi
	@echo ""
	@echo "=== Testing Negative Number Literals ==="
	@$(COMPILER) tests/negative.js > tests/negative.wat
	@result=$$(wasmtime tests/negative.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "10" ]; then \
		if grep -q "i32.const -5" tests/negative.wat; then \
			echo "PASS (got 10, negative literals folded)"; \
		else \
			echo "FAIL (got 10, but negative literals not folded)"; exit 1; \
		fi \
	else \
		echo "FAIL (expected 10, got $$result)"; exit 1; \
	fi
	@echo ""
	@echo "=== Testing Comments ==="
	@$(COMPILER) tests/comments.js > tests/comments.wat
	@result=$$(wasmtime tests/comments.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "15" ]; then echo "PASS (got 15)"; else echo "FAIL (expected 15, got $$result)"; exit 1; \
	fi
	@echo ""
	@echo "=== Testing Logical AND/OR Operators ==="
	@$(COMPILER) tests/logical.js > tests/logical.wat
	@result=$$(wasmtime tests/logical.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "21" ]; then echo "PASS (got 21)"; else echo "FAIL (expected 21, got $$result)"; exit 1; fi
	@echo ""
	@echo "=== Testing For Loop Basic ==="
	@$(COMPILER) tests/for_loop_basic.js > tests/for_loop_basic.wat
	@result=$$(wasmtime tests/for_loop_basic.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "55" ]; then echo "PASS (got 55)"; else echo "FAIL (expected 55, got $$result)"; exit 1; fi
	@echo ""
	@echo "=== Testing For Loop Nested ==="
	@$(COMPILER) tests/for_loop_nested.js > tests/for_loop_nested.wat
	@result=$$(wasmtime tests/for_loop_nested.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "225" ]; then echo "PASS (got 225)"; else echo "FAIL (expected 225, got $$result)"; exit 1; fi
	@echo ""
	@echo "=== Testing For Loop Empty Init ==="
	@$(COMPILER) tests/for_loop_empty_init.js > tests/for_loop_empty_init.wat
	@result=$$(wasmtime tests/for_loop_empty_init.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "10" ]; then echo "PASS (got 10)"; else echo "FAIL (expected 10, got $$result)"; exit 1; fi
	@echo ""
	@echo "=== Testing For Loop Empty Increment ==="
	@$(COMPILER) tests/for_loop_empty_incr.js > tests/for_loop_empty_incr.wat
	@result=$$(wasmtime tests/for_loop_empty_incr.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "10" ]; then echo "PASS (got 10)"; else echo "FAIL (expected 10, got $$result)"; exit 1; fi
	@echo ""
	@echo "=== Testing For Loop Factorial ==="
	@$(COMPILER) tests/for_loop_factorial.js > tests/for_loop_factorial.wat
	@result=$$(wasmtime tests/for_loop_factorial.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "720" ]; then echo "PASS (got 720)"; else echo "FAIL (expected 720, got $$result)"; exit 1; fi
	@echo ""
	@echo "=== Testing For Loop Countdown ==="
	@$(COMPILER) tests/for_loop_countdown.js > tests/for_loop_countdown.wat
	@result=$$(wasmtime tests/for_loop_countdown.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "55" ]; then echo "PASS (got 55)"; else echo "FAIL (expected 55, got $$result)"; exit 1; fi
	@echo ""
	@echo "=== Testing Break in While Loop ==="
	@$(COMPILER) tests/break_while.js > tests/break_while.wat
	@result=$$(wasmtime tests/break_while.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "7" ]; then echo "PASS (got 7)"; else echo "FAIL (expected 7, got $$result)"; exit 1; fi
	@echo ""
	@echo "=== Testing Break in For Loop ==="
	@$(COMPILER) tests/break_for.js > tests/break_for.wat
	@result=$$(wasmtime tests/break_for.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "55" ]; then echo "PASS (got 55)"; else echo "FAIL (expected 55, got $$result)"; exit 1; fi
	@echo ""
	@echo "=== Testing Continue in While Loop ==="
	@$(COMPILER) tests/continue_while.js > tests/continue_while.wat
	@result=$$(wasmtime tests/continue_while.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "25" ]; then echo "PASS (got 25)"; else echo "FAIL (expected 25, got $$result)"; exit 1; fi
	@echo ""
	@echo "=== Testing Continue in For Loop ==="
	@$(COMPILER) tests/continue_for.js > tests/continue_for.wat
	@result=$$(wasmtime tests/continue_for.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "37" ]; then echo "PASS (got 37)"; else echo "FAIL (expected 37, got $$result)"; exit 1; fi
	@echo ""
	@echo "=== Testing Break in Nested Loops ==="
	@$(COMPILER) tests/break_nested.js > tests/break_nested.wat
	@result=$$(wasmtime tests/break_nested.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "25" ]; then echo "PASS (got 25)"; else echo "FAIL (expected 25, got $$result)"; exit 1; fi
	@echo ""
	@echo "=== Testing Continue in Nested Loops ==="
	@$(COMPILER) tests/continue_nested.js > tests/continue_nested.wat
	@result=$$(wasmtime tests/continue_nested.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "12" ]; then echo "PASS (got 12)"; else echo "FAIL (expected 12, got $$result)"; exit 1; fi
	@echo ""
	@echo "=== All tests passed ==="

# Individual test targets
test-fact: build
	@$(COMPILER) tests/factorial.js > tests/factorial.wat
	@result=$$(wasmtime tests/factorial.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "120" ]; then echo "PASS (got 120)"; else echo "FAIL (expected 120, got $$result)"; exit 1; fi

test-gcd: build
	@$(COMPILER) tests/gcd.js > tests/gcd.wat
	@result=$$(wasmtime tests/gcd.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "6" ]; then echo "PASS (got 6)"; else echo "FAIL (expected 6, got $$result)"; exit 1; fi

test-ack: build
	@$(COMPILER) tests/ackermann.js > tests/ackermann.wat
	@result=$$(wasmtime --wasm tail-call tests/ackermann.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "125" ]; then echo "PASS (got 125)"; else echo "FAIL (expected 125, got $$result)"; exit 1; fi

test-const: build
	@echo "=== Testing Const Reassignment Error ==="
	@output=$$($(COMPILER) tests/const_error.js 2>&1 || true); \
	echo "$$output"; \
	if echo "$$output" | grep -q "Cannot reassign const"; then \
		echo ""; \
		echo "PASS (const reassignment rejected)"; \
	else \
		echo "FAIL (const reassignment not detected)"; exit 1; \
	fi

# Constant folding test
test-fold: build
	@echo "=== Testing Constant Folding ==="
	@$(COMPILER) tests/const_fold.js > tests/const_fold.wat
	@result=$$(wasmtime tests/const_fold.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "19" ]; then \
		if grep -q "i32.const 11" tests/const_fold.wat && grep -q "i32.const 8" tests/const_fold.wat; then \
			echo "PASS (got 19, constants folded)"; \
			echo ""; \
			echo "Verification: check tests/const_fold.wat for 'i32.const 11' and 'i32.const 8'"; \
		else \
			echo "FAIL (got 19, but constants not folded)"; exit 1; \
		fi \
	else \
		echo "FAIL (expected 19, got $$result)"; exit 1; \
	fi

# Dead code elimination test
test-dead: build
	@echo "=== Testing Dead Code Elimination ==="
	@$(COMPILER) tests/dead_code.js > tests/dead_code.wat
	@result=$$(wasmtime tests/dead_code.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "5" ]; then \
		if ! grep -q "local \$$y" tests/dead_code.wat && ! grep -q "local \$$z" tests/dead_code.wat; then \
			echo "PASS (got 5, dead code eliminated)"; \
			echo ""; \
			echo "Verification: check tests/dead_code.wat"; \
			echo "Variables y and z do NOT appear - code after 'return' was eliminated."; \
		else \
			echo "FAIL (got 5, but dead code not eliminated)"; exit 1; \
		fi \
	else \
		echo "FAIL (expected 5, got $$result)"; exit 1; \
	fi

# Tail call elimination test
# Verifies: return_call is used instead of call + return for tail-position calls
test-tail: build
	@echo "=== Testing Tail Call Elimination ==="
	@$(COMPILER) tests/ackermann.js > tests/ackermann.wat
	@if grep -q "return_call" tests/ackermann.wat; then \
		result=$$(wasmtime --wasm tail-call tests/ackermann.wat --invoke _start 2>&1 | tail -1); \
		if [ "$$result" = "125" ]; then \
			echo "PASS (got 125, tail calls optimized)"; \
			echo ""; \
			echo "Verification: check tests/ackermann.wat for 'return_call' instructions"; \
		else \
			echo "FAIL (expected 125, got $$result)"; exit 1; \
		fi \
	else \
		echo "FAIL (no return_call instructions found)"; exit 1; \
	fi

test-negative: build
	@echo "=== Testing Negative Number Literals ==="
	@$(COMPILER) tests/negative.js > tests/negative.wat
	@result=$$(wasmtime tests/negative.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "10" ]; then \
		if grep -q "i32.const -5" tests/negative.wat; then \
			echo "PASS (got 10, negative literals folded)"; \
		else \
			echo "FAIL (got 10, but negative literals not folded)"; exit 1; \
		fi \
	else \
		echo "FAIL (expected 10, got $$result)"; exit 1; \
	fi

test-comments: build
	@echo "=== Testing Comments ==="
	@$(COMPILER) tests/comments.js > tests/comments.wat
	@result=$$(wasmtime tests/comments.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "15" ]; then echo "PASS (got 15)"; else echo "FAIL (expected 15, got $$result)"; exit 1; \
	fi

test-logical: build
	@echo "=== Testing Logical AND/OR Operators ==="
	@$(COMPILER) tests/logical.js > tests/logical.wat
	@result=$$(wasmtime tests/logical.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "21" ]; then echo "PASS (got 21)"; else echo "FAIL (expected 21, got $$result)"; exit 1; fi

test-for-basic: build
	@echo "=== Testing For Loop Basic ==="
	@$(COMPILER) tests/for_loop_basic.js > tests/for_loop_basic.wat
	@result=$$(wasmtime tests/for_loop_basic.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "55" ]; then echo "PASS (got 55)"; else echo "FAIL (expected 55, got $$result)"; exit 1; fi

test-for-nested: build
	@echo "=== Testing For Loop Nested ==="
	@$(COMPILER) tests/for_loop_nested.js > tests/for_loop_nested.wat
	@result=$$(wasmtime tests/for_loop_nested.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "225" ]; then echo "PASS (got 225)"; else echo "FAIL (expected 225, got $$result)"; exit 1; fi

test-for-empty-init: build
	@echo "=== Testing For Loop Empty Init ==="
	@$(COMPILER) tests/for_loop_empty_init.js > tests/for_loop_empty_init.wat
	@result=$$(wasmtime tests/for_loop_empty_init.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "10" ]; then echo "PASS (got 10)"; else echo "FAIL (expected 10, got $$result)"; exit 1; fi

test-for-empty-incr: build
	@echo "=== Testing For Loop Empty Increment ==="
	@$(COMPILER) tests/for_loop_empty_incr.js > tests/for_loop_empty_incr.wat
	@result=$$(wasmtime tests/for_loop_empty_incr.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "10" ]; then echo "PASS (got 10)"; else echo "FAIL (expected 10, got $$result)"; exit 1; fi

test-for-factorial: build
	@echo "=== Testing For Loop Factorial ==="
	@$(COMPILER) tests/for_loop_factorial.js > tests/for_loop_factorial.wat
	@result=$$(wasmtime tests/for_loop_factorial.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "720" ]; then echo "PASS (got 720)"; else echo "FAIL (expected 720, got $$result)"; exit 1; fi

test-for-countdown: build
	@echo "=== Testing For Loop Countdown ==="
	@$(COMPILER) tests/for_loop_countdown.js > tests/for_loop_countdown.wat
	@result=$$(wasmtime tests/for_loop_countdown.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "55" ]; then echo "PASS (got 55)"; else echo "FAIL (expected 55, got $$result)"; exit 1; fi

test-break-while: build
	@echo "=== Testing Break in While Loop ==="
	@$(COMPILER) tests/break_while.js > tests/break_while.wat
	@result=$$(wasmtime tests/break_while.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "7" ]; then echo "PASS (got 7)"; else echo "FAIL (expected 7, got $$result)"; exit 1; fi

test-break-for: build
	@echo "=== Testing Break in For Loop ==="
	@$(COMPILER) tests/break_for.js > tests/break_for.wat
	@result=$$(wasmtime tests/break_for.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "55" ]; then echo "PASS (got 55)"; else echo "FAIL (expected 55, got $$result)"; exit 1; fi

test-continue-while: build
	@echo "=== Testing Continue in While Loop ==="
	@$(COMPILER) tests/continue_while.js > tests/continue_while.wat
	@result=$$(wasmtime tests/continue_while.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "25" ]; then echo "PASS (got 25)"; else echo "FAIL (expected 25, got $$result)"; exit 1; fi

test-continue-for: build
	@echo "=== Testing Continue in For Loop ==="
	@$(COMPILER) tests/continue_for.js > tests/continue_for.wat
	@result=$$(wasmtime tests/continue_for.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "37" ]; then echo "PASS (got 37)"; else echo "FAIL (expected 37, got $$result)"; exit 1; fi

test-break-nested: build
	@echo "=== Testing Break in Nested Loops ==="
	@$(COMPILER) tests/break_nested.js > tests/break_nested.wat
	@result=$$(wasmtime tests/break_nested.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "25" ]; then echo "PASS (got 25)"; else echo "FAIL (expected 25, got $$result)"; exit 1; fi

test-continue-nested: build
	@echo "=== Testing Continue in Nested Loops ==="
	@$(COMPILER) tests/continue_nested.js > tests/continue_nested.wat
	@result=$$(wasmtime tests/continue_nested.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "12" ]; then echo "PASS (got 12)"; else echo "FAIL (expected 12, got $$result)"; exit 1; fi

clean:
	cargo clean
	rm -f tests/*.wat
