import assert from 'node:assert/strict';
import test from 'node:test';
import { BOARD_WIDTH, BOARD_HEIGHT, renderGame } from '../src/render.js';
import { createPiece, SHAPES } from '../src/pieces.js';

test('empty board has ten two-column cells and twenty rows enclosed by borders', () => {
  assert.equal(BOARD_WIDTH, 10);
  assert.equal(BOARD_HEIGHT, 20);
  const rows = renderGame().split('\r\n');
  assert.equal(rows.length, 22);
  assert.equal(rows[0].slice(0, 22), '+--------------------+');
  assert.equal(rows.at(-1), '+--------------------+');
  for (const row of rows.slice(1, -1)) {
    assert.equal(row.slice(0, 22), `|${' '.repeat(20)}|`);
  }
});

test('all previews and statuses fit without scrolling or leaving stale status text', () => {
  for (let i = 0; i < Object.keys(SHAPES).length; i++) {
    const nextPiece = createPiece(() => i / 7);
    const frames = ['running', 'paused', 'over'].map((status) =>
      renderGame({ nextPiece, status, score: 1600 }).split('\r\n'));
    for (const rows of frames) {
      assert.equal(rows.length, 22);
      assert.ok(rows.every((row) => row.length <= 44));
      assert.equal(rows.slice(5, 7).join('').match(/\[\]/g).length, 4);
      assert.equal(rows.at(-1), '+--------------------+');
    }
    for (const index of [18, 19]) {
      assert.equal(frames[0][index].length, frames[1][index].length);
      assert.equal(frames[0][index].length, frames[2][index].length);
      assert.equal(frames[0][index].slice(24).trim(), '');
    }
  }
});

test('score and controls fit beside the board within a standard 80-by-24 terminal', () => {
  const display = renderGame();
  assert.match(display, /TETRIS/);
  assert.match(display, /Score: 0/);
  assert.match(display, /Controls/);
  assert.match(display, /Ctrl\+C to quit/);
  const rows = display.split('\r\n');
  assert.ok(rows.length <= 24);
  assert.ok(rows.every((row) => row.length <= 80));
  assert.match(renderGame({ score: 1234 }), /Score: 1234/);
  assert.ok(!display.endsWith('\n'), 'no trailing newline that could scroll the display');
});
