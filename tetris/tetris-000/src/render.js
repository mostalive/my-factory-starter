export const BOARD_WIDTH = 10;
export const BOARD_HEIGHT = 20;

export function renderGame({ score = 0 } = {}) {
  const border = `+${'-'.repeat(BOARD_WIDTH * 2)}+`;
  const sidebar = ['TETRIS', '', `Score: ${score}`, '', 'Controls', 'Ctrl+C to quit'];
  const rows = [border];
  for (let y = 0; y < BOARD_HEIGHT; y++) {
    rows.push(`|${'  '.repeat(BOARD_WIDTH)}|`);
  }
  rows.push(border);
  return rows.map((row, index) => sidebar[index] ? `${row}  ${sidebar[index]}` : row).join('\r\n');
}
