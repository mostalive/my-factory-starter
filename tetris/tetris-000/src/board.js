import { BOARD_WIDTH, BOARD_HEIGHT } from './render.js';

export function createBoard() {
  return Array.from({ length: BOARD_HEIGHT }, () => Array(BOARD_WIDTH).fill(0));
}

export function collides(board, piece) {
  return piece.shape.some((row, dy) => row.some((cell, dx) => {
    if (!cell) return false;
    const x = piece.x + dx;
    const y = piece.y + dy;
    return x < 0 || x >= BOARD_WIDTH || y < 0 || y >= BOARD_HEIGHT || Boolean(board[y][x]);
  }));
}

// Return the lowered piece, or null once it has settled into the board.
export function dropPiece(board, piece) {
  const lowered = { ...piece, y: piece.y + 1 };
  if (!collides(board, lowered)) return lowered;
  piece.shape.forEach((row, dy) => row.forEach((cell, dx) => {
    if (cell) board[piece.y + dy][piece.x + dx] = cell;
  }));
  return null;
}

// Drop to the landing position and lock immediately, just like repeated soft drops.
export function hardDropPiece(board, piece) {
  while (piece) piece = dropPiece(board, piece);
  return null;
}

// Compact surviving rows downward and return the number cleared.
export function clearRows(board) {
  const remaining = board.filter((row) => !row.every(Boolean));
  const cleared = BOARD_HEIGHT - remaining.length;
  board.splice(0, board.length,
    ...Array.from({ length: cleared }, () => Array(BOARD_WIDTH).fill(0)),
    ...remaining);
  return cleared;
}

export function rotatePiece(board, piece) {
  const shape = piece.shape[0].map((_, x) => piece.shape.map((row) => row[x]).reverse());
  const rotated = { ...piece, shape };
  return collides(board, rotated) ? piece : rotated;
}

export function movePiece(board, piece, dx) {
  const moved = { ...piece, x: piece.x + dx };
  return collides(board, moved) ? piece : moved;
}
