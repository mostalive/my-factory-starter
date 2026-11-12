# Homework coaching handoff

Updated 2026-11-12. Open the agent at the repository root and follow
`AGENTS.md` and `.agents/skills/coach-me/SKILL.md`.

## How to coach

- The student chose Rust and has production experience, but wants syntax,
  ownership, and lifetime explanations as needed.
- The student writes changes by hand. Show one small change, explain it,
  and wait. Do not edit unless explicitly asked; ambiguous remarks are not
  permission to take over.
- Use an XP rhythm: small vertical slices, meaningful failing tests,
  working behavior, then cleanup. Avoid long stretches of fixture setup.
- Reference current files and line numbers, quoting nearby code before
  showing changes. The agent may inspect files and run tests.

## Current state

- `factory/ITERATION` is `001 WIP`. Homework 002 was read for context only;
  it has not been adopted. Do not fetch the next iteration automatically.
- Rust replaced the Go scaffold in commit `07ae4ea`. Rust 1.99.1 was
  available on this machine. Cargo dependencies are locked.
- `factory/src/main.rs` uses Clap for required `--seed` and `--target`
  arguments, then prints `factory not built yet`. No agent loop exists yet.
- `factory/tests/cli.rs`: `missing_target_is_rejected` passes. Clap already
  supplies that behavior; it does not check whether the seed file exists.
- `factory/tests/features.rs` defines two passing setup steps: copying the
  executable into a temporary workspace, and creating a target directory.
  The world also has a seed-path field, but no seed-writing step yet.
- Latest acceptance run: 30 scenarios fail at undefined steps; 60 setup
  steps pass. No complete acceptance scenario passes yet.
- The reference factory previously finished building `tetris-000`. That
  is separate from implementing the student's own factory.

## Next slice

Read the current files first. Coach the student through adding a CLI test
in `factory/tests/cli.rs` that supplies a valid target argument and a
nonexistent seed path. Assert unsuccessful exit and a useful seed error.
Run it to show the current implementation fails, then guide the smallest
factory change that makes it pass. Keep tests isolated with temporary
directories. Preserve the unchanged Gherkin suite as the acceptance goal;
the direct CLI tests are an incremental route toward it.

## Commands and environment

From `factory/`:

```sh
cargo test --test cli
cargo test --test features
cargo test --test features -- --name "No target argument"
cargo test --test features --features real-agent
```

The last command explicitly selects real-agent scenarios. They remain
undefined at present. `bin/factory` launches the CLI from the repository root.

The student uses terminal Doom Emacs inside tmux from a Mac: `emacs -nw`.
In normal mode, Space then `:` opens the command prompt. Use `compile`
with an explicit Cargo command, then `recompile`. Esc then x did not work
reliably for this student. Rust support needs `rust-analyzer` and `rust-src`.

The agent's execution environment had restricted access to shared Cargo
caches and GCC internals. Tests succeeded using a temporary `CARGO_HOME`
and Clang as the linker. This was an agent-environment workaround, not a
required project setting; try normal Cargo commands on a new machine.

There are uncommitted and partly staged student changes, including test
work, `.gitignore`, and reference-factory machine settings. Inspect status
and preserve them; do not commit unrelated files. Commit and push intended
work before moving machines. Editor configuration is outside this repo.
