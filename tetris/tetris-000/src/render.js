export const BOARD_WIDTH = 10;
export const BOARD_HEIGHT = 20;

export function renderGame({ score = 0, activePiece = null, nextPiece = null, board = [], status = 'running' } = {}) {
  const border = `+${'-'.repeat(BOARD_WIDTH * 2)}+`;
  const preview = Array.from({ length: 2 }, (_, y) =>
    Array.from({ length: 4 }, (_, x) => nextPiece?.shape[y]?.[x] ? '[]' : '  ').join(''));
  const sidebar = ['TETRIS', '', `Score: ${score}`, '', 'Next:', ...preview, '',
    'Controls', 'Left/Right: move', 'Up: rotate', 'Down: soft drop', 'Space: hard drop',
    'P: pause/resume', 'R: restart', 'Ctrl+C to quit', '',
    status === 'over' ? 'GAME OVER' : status === 'paused' ? 'PAUSED' : '',
    status === 'over' ? 'R to play again' : status === 'paused' ? 'P to resume' : ''];
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
  return rows.map((row, index) => index < sidebar.length ? `${row}  ${sidebar[index].padEnd(20)}` : row).join('\r\n');
}
