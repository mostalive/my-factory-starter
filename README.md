# my-factory-starter

Build a software factory. Start with one coding agent, and grow a
system you can configure, observe and steer.

This is the starter for the
[Build a Software Factory tutorial](https://github.com/lean-software-production/build-a-software-factory-tutorial).
Fork it to start building your factory.

## What you're building

- **Your fork is your factory.** Keep its name, `my-factory-starter`,
  or give it any name you like. Your capstone project comes later, in a
  folder of its own inside your fork.
- **The homeworks grow a factory**: a program that turns a written spec
  into working software by running coding agents. Over eight homeworks
  it learns to automate (turn a seed into working software, one task at
  a time), compose (configured machines, explicit routes, runs and targets) and
  operate (parallel work you can watch live and steer).
- **Your factory learns on Tetris first.** The homeworks build Tetris,
  in `tetris/`, as a practice target everyone shares. Your capstone
  comes later.

Each homework follows the same loop:

1. **Fetch**: your coding agent downloads one homework into `factory/spec/`.
2. **Build**: make its feature files pass. They are your acceptance tests.
   You choose the language, libraries and design.
3. **Repeat**: fetch again. Your step definitions carry forward.

This guide gets you from nothing to your first homework, open and ready
to work on. Getting set up takes about 15 minutes, most of it waiting
for your Codespace to build. Before the first homework you also watch a
finished factory build a Tetris, which takes about 20 minutes more. We
walk through GitHub Codespaces, where you sign in two agents with your
ChatGPT plan: the Codex extension, which coaches you, and Pi, which your
factory runs. [Other setups](#other-setups) work too.

## Before you begin

- [ ] A GitHub account.
- [ ] A model provider subscription or API key. We recommend a ChatGPT
      plan that includes Codex, and this guide uses one, but other model
      providers work too: see [Other setups](#other-setups).
- [ ] With ChatGPT, device code sign-in turned on for Codex, which Pi
      uses to sign in from a Codespace: in ChatGPT, open
      **Settings → Security** and turn on **Enable device code
      authorization for Codex**. On a Business or Team workspace, your
      workspace admin turns it on under **Permissions & Roles**.

You don't need to install anything on your computer.

## Get started

### 1. Fork the starter

On GitHub, fork this repository. You can keep the name it suggests.

**You should see** your fork at `github.com/<you>/my-factory-starter`.

<!-- screenshot 01-fork.png: the "Create a new fork" page -->

### 2. Open and check

Open your fork in a Codespace, sign in to the Codex extension and Pi,
then check your tools without calling a model.

#### Create a Codespace

On your fork's page, click **Code → Codespaces → Create codespace on
main**. The first build takes a few minutes.

**You should see** VS Code in your browser, with a terminal that ends in
the output of `bin/doctor`. It reports Pi and Codex as installed but not
yet signed in. Those warnings are expected: you sign in next.

<!-- screenshot 02-codespace-menu.png: the Code → Codespaces menu with "Create codespace on main" -->
<!-- screenshot 02-codespace-ready.png: VS Code in the browser, terminal showing the first bin/doctor output -->

#### Sign in to the Codex extension

Open **Codex** from the sidebar and click **Sign in with ChatGPT**.
Finish signing in to ChatGPT in the browser tab it opens.

**You should see** the Codex chat panel, ready for a message.

<!-- screenshot 02-codex-sign-in.png: the Codex sidebar with "Sign in with ChatGPT" -->
<!-- screenshot 02-codex-panel.png: the Codex chat panel after signing in -->

#### Sign in to Pi

Your factory runs Pi to do its work. In the terminal, start Pi:

```sh
pi
```

Type `/login`, choose **OpenAI Codex (legacy)**, then **Device code
login (headless)**. Open the link it prints, sign in to ChatGPT, and
enter the one-time code. The code expires after 15 minutes. Then type
`/quit` to leave Pi.

**You should see** Pi report that you're signed in.

If it fails after you enter the code, device code sign-in is probably
off: check [Before you begin](#before-you-begin).

<!-- screenshot 02-pi-login.png: Pi's /login provider list with "OpenAI Codex (legacy)" -->
<!-- screenshot 02-pi-device-code.png: Pi showing the device code and link -->

#### Check your tools

```sh
bin/doctor --agent codex
```

**You should see** these lines, among others:

```text
PASS Pi authentication is ready for provider openai-codex (native local check; no refresh).
PASS Codex authentication is configured.
PASS Environment is ready for codex.
```

Ready means Bash, Git, Node, npm and all three agent CLIs (Pi, Claude
Code and Codex) are present, and Pi and Codex are signed in.

If it says an agent is not on `PATH`, rebuild the container: open the
Command Palette and run **Codespaces: Rebuild Container**. If Pi or
Codex isn't signed in, repeat its sign-in step.

<!-- screenshot 02-doctor-ready.png: bin/doctor reporting Pi and Codex signed in and the environment ready -->

### 3. Start your coding agent

Your coding agent is the Codex chat panel. It works in your Codespace's
folder, the repository root, which is where it finds its instructions
(`AGENTS.md`) and the course skills. Your factory source will live in
`factory/`.

**You should see** the Codex chat panel open beside your editor.

Codex starts with Full Access: it runs commands, including `git push`,
without asking first. Change that in the panel's permissions setting,
or in `~/.codex/config.toml`, if you'd rather approve each one.

<!-- screenshot 03-codex-panel.png: the Codex chat panel open beside the editor -->

### 4. Build tetris-000 with the reference factory

Before you build a factory, watch a finished one work. The course has a
reference factory: you give it a seed, and it builds a Tetris in
`tetris/tetris-000`. Your own factory will answer to the same commands
by the end of the homeworks. Your coding agent gets it ready, and you
type the commands.

Say, in the Codex chat panel:

> coach me

Before your first homework, your agent offers to build `tetris-000` with
the reference factory. Say yes. If you'd rather skip it and start
homework 1 straight away, say so, and go to
[step 5](#5-fetch-homework-1). You can also ask for it directly at any
time: "use the reference factory to build tetris-000".

Your agent downloads the reference factory into `bin/reference-factory`,
checks it, and makes sure it can use your Pi sign-in. If you signed in
to Pi with a login, it copies that login into the factory's two
machines, `planner` and `ralph`; if you use an API key from the
environment, it copies nothing. The machines use `openai-codex/gpt-6.1-sol`.
If you're signed in to a different provider, your agent offers you a
model of that provider to use instead.

The reference factory has builds for Linux (x86-64 and arm64) and for
Macs with Apple silicon. On an Intel Mac, your agent says there's no
build for your machine. That's fine: skip to
[step 5](#5-fetch-homework-1), and you'll build your own factory
without watching the reference one first.

**You should see** your agent give you two commands to type.

Type them in a terminal at the repository root. The first starts the
run; the second shows it live:

```sh
bin/reference-factory --run tetris --target tetris/tetris-000 --line implement-with-ralph-loop --seed tetris/spec.md
bin/reference-factory watch tetris --target tetris/tetris-000
```

It takes about 20 minutes, longer if it stops and you start it again.
The planner writes a plan, then `ralph` does one task at a time, and the
planner checks the plan again after each. Leave the terminal open. If
you press Ctrl-C, you stop watching but the run carries on; type the
second command again to pick the view back up.

**You should see** the machines' work scroll by, above a status bar of
three rows that stays at the bottom:

```text
tetris on implement-with-ralph-loop · tetris/tetris-000
start ▸ planner ▸ ralph ↺ planner                        round 3
3m 15s · planner 18.9k · ralph 16.7k · run 35.6k tokens
```

The first row names the run and where it builds. The second shows the
line the work follows and the round. The third has the time and the
tokens used. When the run finishes, the terminal says
`run tetris finished` with the time, and gives you the prompt back.

A stop is likely: both of the builds we timed on this model stopped
once with an error, such as "ended its turn having generated nothing".
When a run stops, `watch` ends too, printing "run tetris failed" and
the error. Type **both** commands again, one after the other. The run
carries on with the finished tasks kept. If it fails again within
seconds of starting, ask your coding agent again ("use the reference
factory to build tetris-000"): it copies your Pi login into the
machines again and checks it.

Then play the Tetris it built:

```sh
npm --prefix tetris/tetris-000 start
```

**You should see** a Tetris in your terminal. Press Q to quit.

If you ask your agent to do this again after `tetris-000` is built, it
tells you it's built and gives you the play command. If a run was
interrupted, it tells you to type the same two commands to carry on.

<!-- screenshot 04-reference-factory-commands.png: Codex giving the two commands -->
<!-- screenshot 04-watch.png: the watch view with its three-row status bar -->
<!-- screenshot 04-play.png: the Tetris running in the terminal -->

### 5. Fetch Homework 1

Now talk to your coding agent. These are messages in the Codex chat
panel, not shell commands.

First say:

> fetch iteration

It downloads Homework 1 from the
[course](https://github.com/lean-software-production/build-a-software-factory-tutorial) into
`factory/spec/` and commits it.

**You should see** Codex report the homework it fetched. In the
terminal:

```sh
cat factory/ITERATION
```

prints:

```text
001 WIP
```

<!-- screenshot 05-fetch-iteration.png: Codex reporting the fetched homework -->
<!-- screenshot 05-iteration.png: cat factory/ITERATION printing 001 WIP -->

Then say:

> coach me

It walks you through the homework one passing example at a time. The
first time, it helps you pick a language for your factory and set up a
Gherkin runner, so the homework's feature files run as your tests.

**You should see** Codex summarise Homework 1 and ask which language
you'd like to build your factory in.

<!-- screenshot 05-coach-me.png: Codex starting to coach Homework 1 -->

## You're set up when

- [ ] `bin/doctor --agent codex` reports Pi and Codex signed in, and
      `Environment is ready for codex.`
- [ ] You've built `tetris/tetris-000` with the reference factory and played it, or chosen to skip it.
- [ ] `factory/ITERATION` reads `001 WIP`.
- [ ] `factory/spec/README.md` exists, holding Homework 1.
- [ ] Codex is coaching you through it.

## Keep the learning loop small

- Treat `factory/spec/features/` as the spec, and don't edit the
  fetched spec.
- Take one failing example at a time.
- Build one homework at a time. When its suite passes, say
  **"fetch iteration"** again.

If you'd rather the agent build a homework for you, say
**"implement it"** for a walkthrough and demo, or **"implement fast"**
to just build it.

## Other setups

To use another model provider, sign Pi in to it with `/login` (or an API
key), and coach with an agent that works with that provider, such as
Claude Code with an Anthropic subscription. Check with
`bin/doctor --agent` and that agent (`pi`, `claude` or `codex`); the
rest of the guide is the same.

You can work in a Dev Container on your own computer, or without a
container at all, with any coding agent harness (Pi, Claude Code,
Codex, etc). Clone your fork and check it:

```sh
git clone https://github.com/<you>/my-factory-starter.git
cd my-factory-starter
bin/doctor
```

If anything is missing, ask your agent to set up an environment
equivalent to this repository's Codespace, described in
`.devcontainer/devcontainer.json`. Sign in to Pi too (`pi`, then
`/login`), because your factory runs it. The reference factory also
needs the Pi bridge on your `PATH`; a Codespace has it, but elsewhere
you add this repository's `tools/pi-rpc-acp` folder to `PATH` (for
example, put `export PATH="/path/to/my-factory-starter/tools/pi-rpc-acp:$PATH"`
in your shell's startup file). You're ready when `bin/doctor` reports
`Environment is ready`; then carry on from
[step 3](#3-start-your-coding-agent), starting your agent (`pi`,
`claude` or `codex`) at the repository root.

## Reference

### Where things live

- `factory/`: your factory source, from the first homework onward.
  - `spec/`: the current homework, with `README.md`, `FACTORY.md` and the
    acceptance criteria in `features/`, which are also your tests.
  - `ITERATION`: which homework you're on, and whether it's done.
- `bin/factory`: a symlink to your factory's entry point, created during
  setup after you choose a language. Run it from the repository root.
- `tetris/spec.md`: the practice seed. It ships with the starter, and
  the first homework supplies the same text.
- `tetris/tetris-000/`: the Tetris the reference factory builds in
  step 4. Before it's built it holds only the `implement-with-ralph-loop`
  line, in `.assembly-lines/`. The factory commits what it builds, and
  its run folder, in this repository.
- `tetris/tetris-001/`, `tetris/tetris-002/`: generated practice games, one
  folder per generation. Each keeps its own plan in `.factory/plan.md`
  through homework 4. From homework 5 a target also holds the assembly
  lines that build it, in `.assembly-lines/<line>/`, each with its
  machines and its runs, and a run's plan and record live in its run's
  folder there. The factory commits only the selected target's generated
  work, and its run folders, in this repository.
- Your capstone product: a separate folder later, such as `plant-feeder/`.
- `bin/doctor`, `bin/doctor_test.sh`: starter environment checks.
- `bin/get-reference-factory`, `bin/get-reference-factory_test.sh`:
  download the course's reference factory release, check it and install
  it as `bin/reference-factory`; and the script's tests.
- `bin/reference-factory`: the reference factory, installed by
  `bin/get-reference-factory`. Git ignores it.
- `tools/pi-rpc-acp/`: from homework 7 your factory runs machines as
  ACP agents; this is the bridge that runs pi as one. The devcontainer
  puts it on your `PATH`, and installs the ACP adapters for Claude Code
  and Codex.
- `.agents/skills/`: the skills `use-reference-factory`, `fetch-iteration`,
  `set-up-factory`, `coach-me`, `tutor-me`, `implement-it` and `implement-fast`.

### Run your first factory

After you build homework 1, give every run the seed and its target, from
the repository root. The first run writes the plan, each run after that
does one task, and `--all` finishes the plan:

```sh
bin/factory --seed tetris/spec.md --target tetris/tetris-001        # write the plan
bin/factory --seed tetris/spec.md --target tetris/tetris-001        # do one task
bin/factory --seed tetris/spec.md --target tetris/tetris-001 --all  # finish the plan
npm --prefix tetris/tetris-001 start                                # play the result
```

After homework 2 adds validation, build the same seed in a fresh target:

```sh
bin/factory --seed tetris/spec.md --target tetris/tetris-002 --all
npm --prefix tetris/tetris-002 start
```

The factory commits each generation's plan along with its work. A completed
run leaves the target's work and final plan recorded in Git, without including
unrelated changes.

A new target starts with a new plan; running it again resumes that plan.
Both games stay available for comparison. Targets are plain folders;
Git is initialized only if the target is outside any repository.
`bin/factory` is the entry point you build during the homework.

### Codespaces and Dev Containers

This repository includes a Dev Container for GitHub Codespaces and local Dev
Container users. It provides Node.js plus Pi, Claude Code, and Codex (installed
from the [lean-software-production devcontainer features](https://github.com/lean-software-production/devcontainer-features)).
Open the repository in a container, then authenticate the agent you want to use
as the non-root `node` user; credentials are not included in the image or
repository. The Codex VS Code extension is pinned to `26.1007.21434`, the last
stable release before `26.1021.22027`. That release's Codex panel loads over
3,000 files before it can start, and fails to start or freezes for many users
([openai/codex#56691](https://github.com/openai/codex/issues/56691)). Move the
pin on once a fixed release is out. The extension brings the Codex Audio
extension with it, which can't run in a browser-based Codespace; Codex works
without it.

```sh
# Pick one. Codespaces users can use the device-code flow when browser callback
# login is inconvenient.
pi                 # then enter /login
claude auth login
codex login --device-auth

# Confirm the environment without contacting a model.
bin/doctor
bin/doctor --agent codex
```

`bin/doctor` checks Node.js, Git, Bash, and all three agent CLIs. By default it
requires the tools and at least one configured agent; `--agent pi`,
`--agent claude`, or `--agent codex` checks a specific choice, and
`--agent all` requires every agent to be configured. It does not read credential
contents or make a model request. For Pi, it uses `pi auth check --no-refresh`
against a saved/uniquely identifiable provider; ambiguous Pi configuration is
reported as unknown rather than ready. Once it reports ready, start Pi with `pi`,
Claude Code with `claude`, or Codex with `codex`. Codex defaults to `gpt-6.1-sol`
with Full Access permissions (no sandbox, no approval prompts); change these
with `/model` and `/permissions`, or in `~/.codex/config.toml`. Full Access
means Codex can run any command, including `git push` with the Codespace's
GitHub token, without asking.

### Testing the starter

Run `bin/doctor_test.sh` to exercise `bin/doctor` against fake agent CLIs.

Run `bin/get-reference-factory_test.sh` to exercise
`bin/get-reference-factory` against a fake release in a scratch folder. It
never touches your `bin/reference-factory`.

Run `python3 .agents/skills/fetch-iteration/fetch_test.py` to check fetching
without network access.
