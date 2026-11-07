import assert from 'node:assert/strict';
import test from 'node:test';
import { createPiece, SHAPES } from '../src/pieces.js';
import { BOARD_WIDTH, BOARD_HEIGHT, renderGame } from '../src/render.js';

test('random generation can select each of the seven distinct tetrominoes', () => {
  const types = ['I', 'O', 'T', 'S', 'Z', 'J', 'L'];
  assert.deepEqual(Object.keys(SHAPES), types);
  const shapes = new Set();
  for (const [index, type] of types.entries()) {
    const piece = createPiece(() => (index + 0.5) / types.length);
    assert.equal(piece.type, type);
    assert.deepEqual(piece.shape, SHAPES[type]);
    assert.equal(piece.shape.flat().filter(Boolean).length, 4);
    assert.equal(piece.y, 0);
    assert.equal(piece.x, Math.floor((BOARD_WIDTH - piece.shape[0].length) / 2));
    shapes.add(JSON.stringify(piece.shape));
  }
  assert.equal(shapes.size, 7);
  assert.equal(createPiece(() => 0).type, 'I');
  assert.equal(createPiece(() => 0.999999).type, 'L');
});

test('generated shapes are independent copies', () => {
  const first = createPiece(() => 0);
  first.shape[0][0] = 0;
  assert.equal(createPiece(() => 0).shape[0][0], 1);
  assert.equal(SHAPES.I[0][0], 1);
});

test('each spawned piece renders four blocks inside the unchanged board layout', () => {
  for (let index = 0; index < 7; index++) {
    const activePiece = createPiece(() => index / 7);
    const rows = renderGame({ activePiece }).split('\r\n');
    assert.equal(rows.length, BOARD_HEIGHT + 2);
    assert.equal(rows.join('').match(/\[\]/g).length, 4);
    for (let y = 0; y < BOARD_HEIGHT; y++) {
      assert.equal(rows[y + 1][0], '|');
      assert.equal(rows[y + 1][21], '|');
      for (let x = 0; x < BOARD_WIDTH; x++) {
        const expected = activePiece.shape[y - activePiece.y]?.[x - activePiece.x] ? '[]' : '  ';
        assert.equal(rows[y + 1].slice(1 + x * 2, 3 + x * 2), expected);
      }
    }
    assert.ok(rows.every((row) => row.length <= 80));
  }
});

test('rendering respects an active piece position without changing it', () => {
  const activePiece = { ...createPiece(() => 0), x: 1, y: 7 };
  const before = structuredClone(activePiece);
  const rows = renderGame({ activePiece }).split('\r\n');
  assert.equal(rows[8].slice(0, 22), '|  [][][][]          |');
  assert.deepEqual(activePiece, before);
});
