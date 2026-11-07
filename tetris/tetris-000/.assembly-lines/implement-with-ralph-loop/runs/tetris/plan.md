# Terminal Tetris plan

- [x] Set up a Node.js project with `npm start` that displays a terminal game shell and exits cleanly on Ctrl+C.
- [ ] Render an empty 10-by-20 board with borders and a compact score-and-controls sidebar, keeping the entire display within 24 terminal rows.
- [ ] Add all seven tetromino shapes, randomized piece generation, and rendering of an active piece on the board.
- [ ] Add left/right keyboard movement with collision checks against the board edges and occupied cells.
- [ ] Add keyboard rotation with collision checks so pieces cannot rotate through walls or settled blocks.
- [ ] Add timed gravity, soft drop, piece locking, and spawning of the next piece to make the game continuously playable.
- [ ] Add full-row clearing and score updates, with tests for collisions, locking, and clearing multiple rows.
- [ ] Add hard drop and a next-piece preview, updating the visible controls without exceeding 24 terminal rows.
- [ ] Add pause/resume, game-over detection, and restart controls, with all status messages inside the existing display.
- [ ] Verify the complete game through automated tests and terminal smoke checks, fix display or input issues, and document startup and controls.
