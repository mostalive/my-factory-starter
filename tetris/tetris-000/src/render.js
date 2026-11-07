export const BOARD_WIDTH = 10;
export const BOARD_HEIGHT = 20;

export function renderGame({ score = 0, activePiece = null, board = [] } = {}) {
  const border = `+${'-'.repeat(BOARD_WIDTH * 2)}+`;
  const sidebar = ['TETRIS', '', `Score: ${score}`, '', 'Controls', 'Left/Right: move', 'Ctrl+C to quit'];
  const rows = [border];
  for (let y = 0; y < BOARD_HEIGHT; y++) {
    let cells = '';
    for (let x = 0; x < BOARD_WIDTH; x++) {
      const occupied = board[y]?.[x] || activePiece?.shape[y - activePiece.y]?.[x - activePiece.x];
      cells += occupied ? '[]' : '  ';
    }
    rows.push(`|${cells}|`);
  }
  rows.push(border);
  return rows.map((row, index) => sidebar[index] ? `${row}  ${sidebar[index]}` : row).join('\r\n');
}
