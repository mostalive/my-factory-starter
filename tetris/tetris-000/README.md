# Terminal Tetris

A dependency-free terminal game for Node.js 18 or newer.

## Start

From this directory, run:

```sh
npm start
```

Use an interactive terminal at least 44 columns wide and 24 rows tall (80×24 recommended). The entire game occupies 22 rows, including the 10×20 board, borders, next-piece preview, score, controls, and status messages. Redirected input/output is not supported. No package installation is required.

## Controls

| Key | Action |
| --- | --- |
| Left / Right arrow | Move |
| Up arrow | Rotate clockwise |
| Down arrow | Soft drop one row |
| Space | Hard drop and lock |
| P | Pause / resume |
| R | Restart, including after game over |
| Ctrl+C | Quit and restore the terminal |

Pieces fall every 700 ms. Complete rows disappear and award 100 / 300 / 500 / 800 points for clearing 1 / 2 / 3 / 4 rows at once. Drops do not award extra points. A blocked spawn ends the game. Rotation is rejected when blocked by the walls, floor, or settled pieces; there are no wall kicks.

## Tests

```sh
npm test
```

Tests use Node's built-in test runner and cover piece generation, movement, rotation, gravity, locking, clearing, scoring, preview, pause, restart, game over, display dimensions, and terminal cleanup.
