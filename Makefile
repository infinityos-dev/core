CARGO ?= cargo

.PHONY: build clean test run clippy

build:
	$(CARGO) build

clean:
	$(CARGO) clean

test:
	$(CARGO) test

run:
	$(CARGO) run

check:
	$(CARGO) check

fmt:
	$(CARGO) fmt

clippy:
	$(CARGO) clippy

doc:
	$(CARGO) doc --open
