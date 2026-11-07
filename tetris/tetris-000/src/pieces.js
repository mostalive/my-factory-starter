import { BOARD_WIDTH } from './render.js';

export const SHAPES = {
  I: [[1, 1, 1, 1]],
  O: [[1, 1], [1, 1]],
  T: [[0, 1, 0], [1, 1, 1]],
  S: [[0, 1, 1], [1, 1, 0]],
  Z: [[1, 1, 0], [0, 1, 1]],
  J: [[1, 0, 0], [1, 1, 1]],
  L: [[0, 0, 1], [1, 1, 1]],
};

export function createPiece(random = Math.random) {
  const types = Object.keys(SHAPES);
  const type = types[Math.floor(random() * types.length)];
  const shape = SHAPES[type].map((row) => [...row]);
  return {
    type,
    shape,
    x: Math.floor((BOARD_WIDTH - shape[0].length) / 2),
    y: 0,
  };
}
