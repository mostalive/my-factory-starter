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
