import assert from 'node:assert/strict';
import test from 'node:test';
import { createBoard, collides, movePiece } from '../src/board.js';
import { createPiece, SHAPES } from '../src/pieces.js';
import { BOARD_WIDTH, BOARD_HEIGHT, renderGame } from '../src/render.js';

test('boards have independent empty rows', () => {
  const board = createBoard();
  assert.equal(board.length, BOARD_HEIGHT);
  assert.ok(board.every((row) => row.length === BOARD_WIDTH && row.every((cell) => cell === 0)));
  board[0][0] = 1;
  assert.equal(board[1][0], 0);
  assert.equal(createBoard()[0][0], 0);
});

test('all seven pieces move both ways and stop at the walls', () => {
  const board = createBoard();
  for (let i = 0; i < 7; i++) {
    const original = createPiece(() => i / 7);
    const snapshot = structuredClone(original);
    assert.equal(movePiece(board, original, -1).x, original.x - 1);
    assert.equal(movePiece(board, original, 1).x, original.x + 1);
    assert.deepEqual(original, snapshot);
    for (const direction of [-1, 1]) {
      let piece = original;
      for (let step = 0; step < BOARD_WIDTH; step++) piece = movePiece(board, piece, direction);
      assert.equal(piece.x, direction === -1 ? 0 : BOARD_WIDTH - piece.shape[0].length);
      assert.equal(collides(board, piece), false);
      assert.equal(movePiece(board, piece, direction), piece);
    }
  }
});

test('occupied cells block movement from either side without mutating state', () => {
  for (const direction of [-1, 1]) {
    const board = createBoard();
    const piece = { type: 'O', shape: SHAPES.O, x: 4, y: 8 };
    board[9][direction === -1 ? 3 : 6] = 1;
    const snapshot = structuredClone({ board, piece });
    assert.equal(movePiece(board, piece, direction), piece);
    assert.equal(movePiece(board, piece, -direction).x, piece.x - direction);
    assert.deepEqual({ board, piece }, snapshot);
  }
});

test('collision checks ignore empty shape cells and reject vertical bounds', () => {
  const board = createBoard();
  const piece = { type: 'T', shape: SHAPES.T, x: 3, y: 4 };
  board[4][3] = 1;
  assert.equal(collides(board, piece), false);
  board[4][4] = 1;
  assert.equal(collides(board, piece), true);
  assert.equal(collides(createBoard(), { ...piece, y: -1 }), true);
  assert.equal(collides(createBoard(), { ...piece, y: BOARD_HEIGHT - 1 }), true);
});

test('settled cells and active cells both render within the existing display', () => {
  const board = createBoard();
  board[19][0] = 1;
  const rows = renderGame({ board, activePiece: createPiece() }).split('\r\n');
  assert.equal(rows.length, 22);
  assert.equal(rows[20].slice(0, 3), '|[]');
  assert.equal(rows.join('').match(/\[\]/g).length, 5);
  assert.match(rows.join('\n'), /Left\/Right: move/);
});
