# Convenience targets for driving the factory's Gherkin suite.
#
#   make test    every feature (skips @real-agent examples)
#   make focus   just the example named in FOCUS below
#   make real    every feature, including @real-agent examples
#   make flow    render the step trend from the recorded runs
#
# `make focus` accepts a name override too:
#   make focus FOCUS="No target argument"

FACTORY := factory
CARGO   := cargo test --manifest-path $(FACTORY)/Cargo.toml --test features

# The example you're working on. Update as you make progress.
FOCUS ?= A plan already exists

.PHONY: test focus real flow

test:
	$(CARGO)

focus:
	$(CARGO) -- --name "$(FOCUS)"

real:
	$(CARGO) --features real-agent

# A full `make test` records a point in factory/stats/runs.jsonl; this renders
# them as factory/stats/flow.svg (steps, examples and median open steps).
flow:
	python3 bin/flow
