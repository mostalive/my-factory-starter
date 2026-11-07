import assert from 'node:assert/strict';
import test from 'node:test';
import { clearRows, createBoard, collides, dropPiece, movePiece, rotatePiece } from '../src/board.js';
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

test('all shapes rotate clockwise without mutation and return after four turns', () => {
  const board = createBoard();
  for (let i = 0; i < 7; i++) {
    const original = createPiece(() => i / 7);
    const snapshot = structuredClone(original);
    let piece = rotatePiece(board, original);
    assert.deepEqual(piece.shape, original.shape[0].map((_, x) => original.shape.map((row) => row[x]).reverse()));
    assert.equal(piece.x, original.x);
    assert.equal(piece.y, original.y);
    assert.equal(piece.shape.flat().filter(Boolean).length, 4);
    assert.equal(collides(board, piece), false);
    for (let turn = 1; turn < 4; turn++) piece = rotatePiece(board, piece);
    assert.deepEqual(piece, original);
    assert.deepEqual(original, snapshot);
  }
});

test('rotation rejects walls, the floor, and settled blocks without changing state', () => {
  const vertical = rotatePiece(createBoard(), createPiece(() => 0));
  for (const x of [BOARD_WIDTH - 1, -1]) {
    const piece = { ...vertical, x };
    assert.equal(rotatePiece(createBoard(), piece), piece);
  }
  const low = { ...createPiece(() => 0), y: BOARD_HEIGHT - 1 };
  assert.equal(rotatePiece(createBoard(), low), low);
  const board = createBoard();
  const piece = createPiece(() => 0);
  board[1][piece.x] = 1;
  const snapshot = structuredClone({ board, piece });
  assert.equal(collides(board, piece), false);
  assert.equal(rotatePiece(board, piece), piece);
  assert.deepEqual({ board, piece }, snapshot);
});

test('rotation ignores empty cells in the rotated shape', () => {
  const board = createBoard();
  const piece = { type: 'T', shape: SHAPES.T, x: 3, y: 4 };
  board[6][4] = 1;
  const rotated = rotatePiece(board, piece);
  assert.notEqual(rotated, piece);
  assert.deepEqual(rotated.shape, [[1, 0], [1, 1], [1, 0]]);
});

test('all seven pieces descend, lock at the floor, and leave other cells untouched', () => {
  for (let i = 0; i < 7; i++) {
    const board = createBoard();
    let piece = createPiece(() => i / 7);
    const original = structuredClone(piece);
    piece = dropPiece(board, piece);
    assert.equal(piece.y, 1);
    assert.deepEqual(board, createBoard());
    assert.deepEqual(original, createPiece(() => i / 7));
    while (piece.y + piece.shape.length < BOARD_HEIGHT) piece = dropPiece(board, piece);
    assert.equal(dropPiece(board, piece), null);
    assert.equal(board.flat().filter(Boolean).length, 4);
    for (let y = 0; y < piece.shape.length; y++) {
      for (let x = 0; x < piece.shape[y].length; x++) {
        assert.equal(board[piece.y + y][piece.x + x], piece.shape[y][x]);
      }
    }
  }
});

test('dropping onto settled blocks locks above them without overwriting them', () => {
  const board = createBoard();
  board[10][4] = 7;
  const piece = { type: 'O', shape: SHAPES.O, x: 4, y: 8 };
  assert.equal(dropPiece(board, piece), null);
  assert.deepEqual(board[8].slice(4, 6), [1, 1]);
  assert.deepEqual(board[9].slice(4, 6), [1, 1]);
  assert.equal(board[10][4], 7);
  assert.equal(board.flat().filter(Boolean).length, 5);
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

test('clearing one through four rows compacts survivors and adds independent empty rows', () => {
  for (const count of [1, 2, 3, 4]) {
    const board = createBoard();
    board[10][2] = 7;
    board[19][8] = 3;
    // Include separated full rows to exercise compaction, not just a bottom run.
    for (let i = 0; i < count; i++) board[11 + i * 2].fill(1);
    assert.equal(clearRows(board), count);
    assert.equal(board.length, BOARD_HEIGHT);
    assert.equal(board[10 + count][2], 7);
    assert.equal(board[19][8], 3);
    assert.equal(board.flat().filter(Boolean).length, 2);
    assert.ok(board.slice(0, count).every((row) => row.every((cell) => cell === 0)));
    board[0][0] = 1;
    assert.equal(board[1][0], 0);
    const snapshot = structuredClone(board);
    assert.equal(clearRows(board), 0);
    assert.deepEqual(board, snapshot);
  }
});

test('locking completes two rows which clear before the next spawn', () => {
  const board = createBoard();
  board[18].fill(1);
  board[19].fill(1);
  for (const y of [18, 19]) board[y][4] = board[y][5] = 0;
  const piece = { type: 'O', shape: SHAPES.O, x: 4, y: 18 };
  assert.equal(collides(board, piece), false);
  assert.equal(dropPiece(board, piece), null);
  assert.equal(clearRows(board), 2);
  assert.deepEqual(board, createBoard());
  assert.equal(collides(board, createPiece()), false);
});
