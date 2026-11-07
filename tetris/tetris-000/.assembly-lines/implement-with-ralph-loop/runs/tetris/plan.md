# Terminal Tetris plan

- [x] Set up a Node.js project with `npm start` that displays a terminal game shell and exits cleanly on Ctrl+C.
- [x] Render an empty 10-by-20 board with borders and a compact score-and-controls sidebar, keeping the entire display within 24 terminal rows.
- [x] Add all seven tetromino shapes, randomized piece generation, and rendering of an active piece on the board.
- [x] Add left/right keyboard movement with collision checks against the board edges and occupied cells.
- [x] Add keyboard rotation with collision checks so pieces cannot rotate through walls or settled blocks.
- [x] Add timed gravity, soft drop, piece locking, and spawning of the next piece to make the game continuously playable.
- [x] Add full-row clearing and score updates, with tests for collisions, locking, and clearing multiple rows.
- [x] Add hard drop and a next-piece preview, updating the visible controls without exceeding 24 terminal rows.
- [x] Add pause/resume, game-over detection, and restart controls, with all status messages inside the existing display.
- [x] Verify the complete game through automated tests and terminal smoke checks, fix display or input issues, and document startup and controls.
