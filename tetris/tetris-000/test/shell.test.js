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
  assert.equal(tty.screen().match(/\[\]/g).length, 4);
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
      assert.equal(frame.match(/\[\]/g).length, 4);
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
      assert.equal(frame.match(/\[\]/g).length, 4);
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
    assert.equal(frame().match(/\[\]/g).length, 8);
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
