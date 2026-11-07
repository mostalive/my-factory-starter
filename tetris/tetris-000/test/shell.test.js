import assert from 'node:assert/strict';
import { EventEmitter } from 'node:events';
import { PassThrough } from 'node:stream';
import test from 'node:test';
import { startShell } from '../src/main.js';

function terminal() {
  const input = new PassThrough();
  const output = new PassThrough();
  const signals = new EventEmitter();
  input.isTTY = output.isTTY = true;
  input.isRaw = false;
  input.setRawMode = (raw) => { input.isRaw = raw; };
  let screen = '';
  output.on('data', (data) => { screen += data; });
  return { input, output, signals, screen: () => screen };
}

test('shell renders and Ctrl+C restores the terminal', () => {
  const tty = terminal();
  const stop = startShell(tty);
  assert.equal(tty.input.isRaw, true);
  assert.match(tty.screen(), /TETRIS/);
  assert.equal(tty.screen().match(/\[\]/g).length, 8);
  assert.match(tty.screen(), /Ctrl\+C to quit/);
  tty.input.emit('keypress', '\x03', { ctrl: true, name: 'c' });
  assert.equal(tty.input.isRaw, false);
  assert.equal(tty.input.isPaused(), true);
  assert.ok(tty.screen().endsWith('\x1b[?25h\x1b[?1049l'));
  const screen = tty.screen();
  stop();
  assert.equal(tty.screen(), screen);
  assert.equal(tty.signals.listenerCount('SIGINT'), 0);
  assert.equal(tty.signals.listenerCount('SIGTERM'), 0);
});

test('arrow key input redraws movement and stops at both walls', () => {
  const tty = terminal();
  const stop = startShell(tty);
  try {
    for (const name of ['left', 'right']) {
      for (let i = 0; i < 10; i++) tty.input.write(name === 'left' ? '\x1b[D' : '\x1b[C');
      const frame = tty.screen().split('\x1b[H').at(-1);
      const rows = frame.split('\r\n');
      assert.equal(rows.length, 22);
      assert.equal(frame.match(/\[\]/g).length, 8);
      if (name === 'left') assert.ok(rows.slice(1, 3).some((row) => row.startsWith('|[]')));
      else assert.ok(rows.slice(1, 3).some((row) => row.slice(19, 22) === '[]|'));
      const before = tty.screen();
      tty.input.emit('keypress', '', { name });
      assert.equal(tty.screen(), before);
    }
  } finally {
    stop();
  }
});

test('Up arrow rotates the piece and preserves the compact display', () => {
  const tty = terminal();
  const stop = startShell(tty);
  try {
    const initial = tty.screen().split('\x1b[H').at(-1);
    assert.match(initial, /Up: rotate/);
    for (let turn = 0; turn < 4; turn++) {
      const before = tty.screen();
      tty.input.write('\x1b[A');
      assert.notEqual(tty.screen(), before);
      const frame = tty.screen().split('\x1b[H').at(-1);
      assert.equal(frame.split('\r\n').length, 22);
      assert.equal(frame.match(/\[\]/g).length, 8);
    }
    assert.equal(tty.screen().split('\x1b[H').at(-1), initial);
  } finally {
    stop();
  }
});

test('gravity and soft drop lower pieces, lock them, and spawn another piece', () => {
  const tty = terminal();
  let tick;
  let cleared = false;
  const clock = {
    setInterval(callback, delay) {
      assert.equal(delay, 700);
      tick = callback;
      return 123;
    },
    clearInterval(timer) {
      assert.equal(timer, 123);
      cleared = true;
    },
  };
  const stop = startShell({ ...tty, clock });
  const frame = () => tty.screen().split('\x1b[H').at(-1);
  try {
    const initial = frame();
    tick();
    const afterGravity = frame().split('\r\n');
    assert.equal(afterGravity[1].slice(1, 21).trim(), '');
    assert.equal(afterGravity[2].slice(0, 22), initial.split('\r\n')[1].slice(0, 22));
    tty.input.write('\x1b[B');
    assert.equal(frame().split('\r\n')[3].slice(0, 22), afterGravity[2].slice(0, 22));
    for (let step = 0; step < 18; step++) tick();
    assert.equal(frame().match(/\[\]/g).length, 12);
    assert.equal(frame().split('\r\n').length, 22);
    assert.match(frame(), /Down: soft drop/);
    assert.ok(frame().split('\r\n')[20].includes('[]'));
  } finally {
    stop();
  }
  assert.equal(cleared, true);
  const stoppedScreen = tty.screen();
  tick();
  tty.input.write('\x1b[B');
  assert.equal(tty.screen(), stoppedScreen);
});

test('completed rows update the displayed score cumulatively without adding display rows', (t) => {
  t.mock.method(Math, 'random', () => 1 / 7); // Always spawn O pieces.
  const tty = terminal();
  const clock = { setInterval() { return 1; }, clearInterval() {} };
  const stop = startShell({ ...tty, clock });
  const frame = () => tty.screen().split('\x1b[H').at(-1);
  try {
    for (let round = 1; round <= 2; round++) {
      for (const x of [0, 2, 4, 6, 8]) {
        const direction = x < 4 ? 'left' : 'right';
        for (let step = 0; step < Math.abs(x - 4); step++) {
          tty.input.emit('keypress', '', { name: direction });
        }
        for (let step = 0; step < 19; step++) {
          tty.input.emit('keypress', '', { name: 'down' });
        }
      }
      assert.match(frame(), new RegExp(`Score: ${round * 300}`));
      assert.equal(frame().match(/\[\]/g).length, 8);
      assert.equal(frame().split('\r\n').length, 22);
    }
    tty.input.write('r');
    assert.match(frame(), /Score: 0/);
    assert.equal(frame().match(/\[\]/g).length, 8);
  } finally {
    stop();
  }
});

test('Space locks immediately and promotes the preview, generating one new preview', (t) => {
  const sequence = [0, 1 / 7, 2 / 7, 3 / 7];
  let calls = 0;
  t.mock.method(Math, 'random', () => sequence[calls++]);
  const tty = terminal();
  const clock = { setInterval() { return 1; }, clearInterval() {} };
  const stop = startShell({ ...tty, clock });
  const rows = () => tty.screen().split('\x1b[H').at(-1).split('\r\n');
  try {
    assert.equal(calls, 2);
    assert.equal(rows()[5].slice(24).trim(), '[][]'); // O preview
    tty.input.write(' ');
    assert.equal(calls, 3);
    assert.equal(rows()[1].slice(1, 21), '        [][]        '); // O active
    assert.equal(rows()[20].slice(1, 21), '      [][][][]      '); // I locked
    assert.equal(rows()[5].slice(24).trim(), '[]'); // T preview, top row
    assert.equal(rows()[6].slice(24).trim(), '[][][]');
    assert.match(rows().join('\n'), /Space: hard drop/);
    assert.equal(rows().length, 22);
    assert.ok(rows().every((row) => row.length <= 80));
  } finally {
    stop();
  }
});

test('pause freezes gravity and gameplay input, and resume continues the same piece', () => {
  const tty = terminal();
  let tick;
  const clock = { setInterval(callback) { tick = callback; return 1; }, clearInterval() {} };
  const stop = startShell({ ...tty, clock });
  const frame = () => tty.screen().split('\x1b[H').at(-1);
  try {
    const initial = frame();
    tty.input.write('p');
    assert.match(frame(), /PAUSED/);
    const paused = tty.screen();
    tick();
    for (const name of ['left', 'right', 'up', 'down', 'space']) {
      tty.input.emit('keypress', '', { name });
    }
    assert.equal(tty.screen(), paused);
    tty.input.write('p');
    assert.equal(frame(), initial);
    tick();
    assert.notEqual(frame(), initial);
    tty.input.write('p');
    tty.input.write('r');
    assert.doesNotMatch(frame(), /PAUSED/);
    assert.match(frame(), /Score: 0/);
    assert.equal(frame().match(/\[\]/g).length, 8);
  } finally {
    stop();
  }
});

test('blocked spawn ends play, ignores gameplay, and restart clears the board', (t) => {
  t.mock.method(Math, 'random', () => 1 / 7); // Stack ten O pieces in the spawn columns.
  const tty = terminal();
  let tick;
  const clock = { setInterval(callback) { tick = callback; return 1; }, clearInterval() {} };
  const stop = startShell({ ...tty, clock });
  const frame = () => tty.screen().split('\x1b[H').at(-1);
  try {
    const initial = frame();
    for (let i = 0; i < 9; i++) {
      tty.input.write(' ');
      assert.doesNotMatch(frame(), /GAME OVER/);
    }
    tty.input.write(' ');
    assert.match(frame(), /GAME OVER/);
    assert.match(frame(), /R to play again/);
    assert.equal(frame().split('\r\n').length, 22);
    assert.ok(frame().split('\r\n').every((row) => row.length <= 80));
    const ended = tty.screen();
    tick();
    for (const name of ['left', 'right', 'up', 'down', 'space', 'p']) {
      tty.input.emit('keypress', '', { name });
    }
    assert.equal(tty.screen(), ended);
    tty.input.write('r');
    assert.equal(frame(), initial);
    tick();
    assert.notEqual(frame(), initial);
    tty.input.write('p');
    tty.input.emit('keypress', '\x03', { ctrl: true, name: 'c' });
    assert.equal(tty.input.isRaw, false);
  } finally {
    stop();
  }
});

test('termination signals also restore the previous raw mode', () => {
  const tty = terminal();
  tty.input.isRaw = true;
  startShell(tty);
  tty.signals.emit('SIGTERM');
  assert.equal(tty.input.isRaw, true);
  assert.ok(tty.screen().endsWith('\x1b[?25h\x1b[?1049l'));
});

test('non-interactive startup reports a useful error without changing the terminal', () => {
  const tty = terminal();
  tty.input.isTTY = false;
  assert.throws(() => startShell(tty), /interactive terminal/);
  assert.equal(tty.screen(), '');
  assert.equal(tty.input.isRaw, false);
});
