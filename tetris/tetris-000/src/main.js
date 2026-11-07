import { emitKeypressEvents } from 'node:readline';
import { pathToFileURL } from 'node:url';

export function startShell({ input = process.stdin, output = process.stdout, signals = process } = {}) {
  if (!input.isTTY || !output.isTTY) {
    throw new Error('Tetris needs an interactive terminal. Run npm start in a terminal.');
  }

  const wasRaw = Boolean(input.isRaw);
  let stopped = false;
  const stop = () => {
    if (stopped) return;
    stopped = true;
    input.off('keypress', onKey);
    input.off('end', stop);
    signals.off('SIGINT', stop);
    signals.off('SIGTERM', stop);
    input.setRawMode(wasRaw);
    input.pause();
    output.write('\x1b[?25h\x1b[?1049l');
  };
  const onKey = (_text, key) => {
    if (key?.ctrl && key.name === 'c') stop();
  };

  emitKeypressEvents(input);
  input.on('keypress', onKey);
  input.once('end', stop);
  signals.once('SIGINT', stop);
  signals.once('SIGTERM', stop);
  input.setRawMode(true);
  input.resume();
  output.write('\x1b[?1049h\x1b[?25l\x1b[2J\x1b[H');
  output.write('TETRIS\r\n\r\nTerminal game ready.\r\nCtrl+C to quit.');
  return stop;
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  try {
    startShell();
  } catch (error) {
    console.error(error.message);
    process.exitCode = 1;
  }
}
