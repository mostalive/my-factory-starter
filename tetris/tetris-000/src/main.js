import { emitKeypressEvents } from 'node:readline';
import { pathToFileURL } from 'node:url';
import { renderGame } from './render.js';
import { createPiece } from './pieces.js';
import { createBoard, movePiece } from './board.js';

export function startShell({ input = process.stdin, output = process.stdout, signals = process } = {}) {
  if (!input.isTTY || !output.isTTY) {
    throw new Error('Tetris needs an interactive terminal. Run npm start in a terminal.');
  }

  const wasRaw = Boolean(input.isRaw);
  let stopped = false;
  const board = createBoard();
  let activePiece = createPiece();
  const draw = () => output.write(`\x1b[H${renderGame({ board, activePiece })}`);
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
    if (key?.ctrl && key.name === 'c') return stop();
    if (key?.name === 'left' || key?.name === 'right') {
      const moved = movePiece(board, activePiece, key.name === 'left' ? -1 : 1);
      if (moved !== activePiece) {
        activePiece = moved;
        draw();
      }
    }
  };

  emitKeypressEvents(input);
  input.on('keypress', onKey);
  input.once('end', stop);
  signals.once('SIGINT', stop);
  signals.once('SIGTERM', stop);
  input.setRawMode(true);
  input.resume();
  output.write('\x1b[?1049h\x1b[?25l\x1b[2J\x1b[H');
  draw();
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
