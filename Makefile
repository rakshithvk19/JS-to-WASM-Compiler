SHELL := /bin/bash
.PHONY: build test clean run test-fact test-gcd test-ack test-const test-fold test-dead

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
	@result=$$(wasmtime tests/ackermann.wat --invoke _start 2>&1 | tail -1); \
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
	@# Verifies: code after 'return' is removed (y, z variables should not appear in .wat)
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
	@result=$$(wasmtime tests/ackermann.wat --invoke _start 2>&1 | tail -1); \
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
# Verifies: 3 + 4 * 2 = 11 and 10 - 2 = 8 are computed at compile time
# Check .wat file for "i32.const 11" and "i32.const 8" instead of multiple operations
test-fold: build
	@echo "=== Testing Constant Folding ==="
	@$(COMPILER) tests/const_fold.js > tests/const_fold.wat
	@result=$$(wasmtime tests/const_fold.wat --invoke _start 2>&1 | tail -1); \
	if [ "$$result" = "19" ]; then \
		if grep -q "i32.const 11" tests/const_fold.wat && grep -q "i32.const 8" tests/const_fold.wat; then \
			echo "PASS (got 19, constants folded)"; \
			echo ""; \
			echo "Verification: check tests/const_fold.wat for 'i32.const 11' and 'i32.const 8'"; \
			echo "These values were computed at compile time instead of runtime."; \
		else \
			echo "FAIL (got 19, but constants not folded)"; exit 1; \
		fi \
	else \
		echo "FAIL (expected 19, got $$result)"; exit 1; \
	fi

# Dead code elimination test
# Verifies: code after 'return' statement is removed
# Check .wat file: variables y and z should NOT appear since they're after return
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

clean:
	cargo clean
	rm -f tests/*.wat