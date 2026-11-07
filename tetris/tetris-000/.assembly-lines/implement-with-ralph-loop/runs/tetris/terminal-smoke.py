"""Run from the product directory: python3 <path-to-this-file>."""
import errno
import fcntl
import os
import pty
import select
import struct
import subprocess
import termios
import time

master, slave = pty.openpty()
fcntl.ioctl(slave, termios.TIOCSWINSZ, struct.pack('HHHH', 24, 80, 0, 0))
before = termios.tcgetattr(slave)
process = subprocess.Popen(['npm', 'start'], stdin=slave, stdout=slave, stderr=slave)
captured = bytearray()


def drain(seconds=0.15):
    deadline = time.monotonic() + seconds
    while time.monotonic() < deadline:
        if select.select([master], [], [], max(0, deadline - time.monotonic()))[0]:
            try:
                captured.extend(os.read(master, 65536))
            except OSError as error:
                if error.errno != errno.EIO:
                    raise
                break


def frame():
    return bytes(captured).split(b'\x1b[H')[-1]


try:
    drain(1.5)
    assert b'TETRIS' in captured, captured
    assert b'\x1b[?1049h' in captured
    os.write(master, b'p')
    drain()
    paused = frame()
    assert b'PAUSED' in paused
    drain(0.85)
    assert frame() == paused, 'gravity changed paused screen'
    os.write(master, b'p\x1b[D\x1b[C\x1b[A\x1b[B ')
    drain()
    assert b'PAUSED' not in frame()
    os.write(master, b'r')
    drain()
    assert b'Score: 0' in frame()
    assert frame().count(b'[]') == 8
    frames = bytes(captured).split(b'\x1b[H')[2:]
    for display in frames:
        rows = display.split(b'\r\n')
        assert len(rows) == 22, len(rows)
        assert all(len(row) <= 80 for row in rows)
    os.write(master, b'\x03')
    process.wait(timeout=3)
    drain()
    assert process.returncode == 0, process.returncode
    assert b'\x1b[?25h\x1b[?1049l' in captured, repr(bytes(captured)[-200:])
    assert termios.tcgetattr(slave) == before, 'terminal settings not restored'
    print(f'PASS: npm start in 80x24 PTY; {len(frames)} frames within 22 rows; arrows, rotation, soft/hard drop, pause/resume, restart, Ctrl+C and terminal restoration.')
finally:
    if process.poll() is None:
        process.terminate()
        process.wait(timeout=3)
    os.close(master)
    os.close(slave)
