# Final verification

- `npm test`: all 30 tests passed.
- `python3 .assembly-lines/implement-with-ralph-loop/runs/tetris/terminal-smoke.py`: passed against `npm start` in a real 80×24 pseudo-terminal. Checked frame dimensions, arrows, rotation, soft/hard drop, pause/resume, restart, Ctrl+C exit, cursor/alternate-screen cleanup, and restoration of terminal attributes.
- Added a regression test covering every next-piece preview in running, paused, and game-over states, including fixed-width status erasure and 22-row/44-column bounds.
- Added `README.md` with Node requirement, startup, terminal size, controls, scoring, and tests.
- No product display or input defects found; no gameplay changes needed. The smoke check looks for cleanup sequences within output rather than at its end because the npm wrapper can append output after the child exits.
