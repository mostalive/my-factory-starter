---
name: coach-me
description: Walk the student through their next homework iteration. Use when the student says "coach me", asks to be coached, or wants to work through their next homework with guidance. It hands the one-example-at-a-time work to the tutor-me skill. Before the first homework, it offers to build tetris-000 with the reference factory, which the student can skip.
---
Walk the student through implementing their next homework iteration.

The student is building a "factory" — a small program that turns a seed into a plan, then works the plan one task at a time, driving a coding agent to do the real work. The seed and plan live in files on disk, not in the factory's code.

## Where things live

Open the agent at the repository root. The factory source, `spec/` and `ITERATION` live in `factory/` for every iteration. Paths below are relative to `factory/` unless stated otherwise. Run the factory CLI from the repository root through `bin/factory`.

- **The factory** — the student's code, in this folder.
- `spec/` — the iteration they're working on, fetched from the course: `README.md`, `FACTORY.md` and `features/`. The feature files are also the factory's test suite.
- `ITERATION` — their progress: one line, the iteration and its status, e.g. `001 WIP`.
- `tetris/spec.md` (from the repository root) — the practice seed passed with `--seed`.
- **The step definitions** — the student's, in their factory project. They tie the feature files to the factory, and make the machines in each example do what it says, with test doubles the student writes (homework 1's README explains).
- **The codebase** — the folder passed with `--target`, such as `tetris/tetris1/` or `tetris/tetris2/` (from the repository root). The factory builds the game there; you don't. The current feature specs define target behavior and where plans and other state live.

## Before the first homework: tetris-000

Only when `ITERATION` doesn't exist yet, so the student hasn't started homework 1, check `tetris/tetris-000` (from the repository root) before the process below. It is built when `tetris/tetris-000/.assembly-lines/implement-with-ralph-loop/runs/tetris/record.jsonl` exists and its last line is the run finished (`"kind":"run"`, `"state":"finished"`).

- If it is built, go on to the process below.
- If its run has started but isn't finished, say so: they can carry on with it (follow the use-reference-factory skill, which gives the same two commands to resume it) or skip it and start homework 1 now.
- If it hasn't been started, offer to build it first: they'd watch the finished reference factory build a Tetris, typing two commands, in about 20 minutes. Or they can skip it and start homework 1 now.
  - If they want it, follow the use-reference-factory skill and stop there. When they say "coach me" again, after playing it, this check finds it built.
  - If they'd rather skip it, go on to the process below. Fetching homework 1 creates `ITERATION`, so you won't offer again.

## Coaching

Follow this process exactly:

1. Read `ITERATION`. If it is missing or says `Done`, follow the fetch-iteration skill first. Carry on here once it has committed the adoption.
2. Read the whole spec:
   - `spec/README.md` — the homework framing and its ground rules
   - `spec/FACTORY.md` — a short summary of the factory at this point
   - `spec/features/` — the acceptance criteria, in Gherkin
   - the seed at `tetris/spec.md` relative to the repository root

   Together they are the whole spec, not just what's new.
3. If the spec is unclear, stop and ask before editing.
4. Check the working tree and avoid touching unrelated student changes.
5. If there is no factory project yet — no Gherkin runner set up in this folder — follow the set-up-factory skill first. For later iterations, keep using what the student already chose. The repository-root `AGENTS.md` says how to run the suite; if it doesn't, work it out from the project.
6. Run the suite, leaving out the `@real-agent` examples. Note which examples fail and which steps are undefined: that is this iteration's work. If nothing fails and no steps are undefined, the suite already passes: skip to step 9.
7. Introduce the iteration with a very concise overview:
   - Goal: the behaviour to add, in plain language (from the README and `FACTORY.md`, not the Gherkin).
   - Where the suite stands: how many examples pass, fail, or have undefined steps.
   - Show the README's example CLI commands and expected outputs, then ask the student what questions they have.
   - Mention that if they'd rather you build the whole iteration, they can use `implement-it` (you build it, then demo it) or `implement-fast` (you just build it).
8. Work through the iteration with the **tutor-me** skill. Follow it from its plan step, so the student gets one example and one instruction at a time, in the intention, location, details pattern. Do not duplicate its per-example guidance here. Coach from its plan step to its last one, then carry on below once every example except the `@real-agent` ones passes.
9. Offer the `@real-agent` examples as something to try by hand, with `pi`, using the README's example commands. They are optional.
10. Finish by pointing at what's still missing. Read the closing lines of `spec/README.md` for what the next homework builds on, and make clear that any remaining rough edges are expected at this point.
11. Once the suite passes, check the iteration description in `spec/README.md` for optional follow-up work before marking the iteration Done:
    - Look for the exact heading `## Once your suite passes`. If present, offer to work through that section, including creating an HTML comparison report when the README suggests one. This heading is a shared contract with the tutorial repository; course content stays in the README.
    - If the description includes a bonus round, show the student its goal and tasks, and ask whether they would like to try it. Use the README's bonus round rather than inventing one.
    - For each offer, wait for the student's answer. If they accept, coach them through the course's tasks in small, student-led steps; if they decline, skip that activity. If neither section is present, continue directly. Skipping optional work does not prevent marking the iteration Done.
12. Change `ITERATION` from `<iteration> WIP` to `<iteration> Done`.
13. Commit the implementation, the step definitions and that change with message `Implement homework <iteration>` (e.g. `Implement homework 003`).
14. Encourage the student to take a look around their factory's codebase. Offer to answer any questions about it, and ask whether there is any refactoring they would like to do.
15. Stop and wait for the student's answer. Answer their questions and coach them through any requested refactoring in the same small, student-led steps.

## Rules

- Do not start more than one iteration.
- Do not edit anything in `spec/`. It is the spec, not yours to change. `ITERATION` is yours to update.
- Break the implementation into small baby steps. That loop, one example at a time, is the tutor-me skill; keep it there, not here.
- Keep the factory minimal: a short loop driving the agent, with state on disk.
  - The intelligence lives in the agent and the files, not the factory.
  - Do not encode the plan or the seed in code.
  - Do not build a framework.
  - Do not add defensive code or hardening. This is a learning exercise.
- The factory never contains an agent of its own, test double or otherwise. Doubles live with the step definitions and are chosen from outside, the same way `pi` is.
- You may run commands to inspect files, review diffs, and run checks.
- Do not commit unrelated existing changes.
- Be super concise. Avoid jargon.
- When introducing an iteration, include only the goal and the steps to get there.
- The suite is the check. Don't test by hand what an example already covers.
- If the suite fails and you cannot coach or implement a fix within the spec, stop and report the failure.
