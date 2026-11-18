# Convenience targets for driving the factory's Gherkin suite.
#
#   make test    every feature (skips @real-agent examples)
#   make focus   just the example named in FOCUS below
#   make real    every feature, including @real-agent examples
#
# `make focus` accepts a name override too:
#   make focus FOCUS="No target argument"

FACTORY := factory
CARGO   := cargo test --manifest-path $(FACTORY)/Cargo.toml --test features

# The example you're working on. Update as you make progress.
FOCUS ?= No target argument

.PHONY: test focus real

test:
	$(CARGO)

focus:
	$(CARGO) -- --name "$(FOCUS)"

real:
	$(CARGO) --features real-agent
