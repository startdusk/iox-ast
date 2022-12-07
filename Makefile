.PHONY: codeline
codeline:
	@tokei .

.PHONY: test 
test: fmt
	@cargo nextest run

.PHONY: fmt
fmt:
	@cargo clippy

.PHONY: lox-ast/r
lox-ast/r: fmt
	@cargo build --release && $(CARGO_TARGET_DIR)/release/lox-ast

.PHONY: lox-ast/pid
lox-ast/pid:
	@pgrep -af lox-ast

.PHONY: lox-ast/test
lox-ast/test:
	@cargo nextest
