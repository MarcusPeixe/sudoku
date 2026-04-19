# CLI Sudoku Specification

These are the specifications for the Sudoku.

## Movements

These are the possible movements:

| Input       | Action                              |
|-------------|-------------------------------------|
| Arrow keys  | Move cursor                         |
| WASD        | Move cursor                         |
| 1-9         | Pick digit and highlight it         |
| 0           | Clear digit                         |
| L           | Write selected digit                |
| K           | Write selected digit in pencil mode |
| Mouse       | Move cursor                         |
| Left click  | Write selected digit                |
| Shift click | Write selected digit in pencil mode |

## Colors

These are the colors that each cell can have:

| Thing                | Ground     | Color   | Code |
|----------------------|------------|---------|------|
| Pre-placed digits    | Foregroung | White   | 37   |
| Player-placed digits | Foregroung | Blue    | 34   |
| Conflicting digits   | Foregroung | Red     | 31   |
| Highlighted digits   | Foregroung | Green   | 32   |
| Board                | Backgroung | Default | 49   |
| Highlighted slots    | Backgroung | Black   | 40   |

## Digits

This is the ASCII art for each digit:

```
. . . . . . . . . . . . .
.   ▄   . ▄▄▄▄▄ . ▄▄▄▄▄ .
.  ▀█   . ▄▄▄▄█ .  ▄▄▄█ .
.  ▄█▄  . █▄▄▄▄ . ▄▄▄▄█ .
. . . . . . . . . . . . .
. ▄   ▄ . ▄▄▄▄▄ . ▄▄▄▄▄ .
. █▄▄▄█ . █▄▄▄▄ . █▄▄▄▄ .
.     █ . ▄▄▄▄█ . █▄▄▄█ .
. . . . . . . . . . . . .
. ▄▄▄▄▄ . ▄▄▄▄▄ . ▄▄▄▄▄ .
. ▀   █ . █▄▄▄█ . █▄▄▄█ .
.     █ . █▄▄▄█ . ▄▄▄▄█ .
. . . . . . . . . . . . .
```

## Pencil markings

This is how pencil markings are rendered:

```
 1 2 3      2      1   3    1 2 3    1   3
 4 5 6    4   6      5      4   6
 7 8 9      8      7   9    7 8 9    7   9
```

## Rendering modes

Board in normal (big) mode:

```
  ▄    ▄▄▄▄▄  ▄▄▄▄▄  ▄   ▄  ▄▄▄▄▄  ▄▄▄▄▄  ▄▄▄▄▄  ▄▄▄▄▄  ▄▄▄▄▄
 ▀█    ▄▄▄▄█   ▄▄▄█  █▄▄▄█  █▄▄▄▄  █▄▄▄▄  ▀   █  █▄▄▄█  █▄▄▄█
 ▄█▄   █▄▄▄▄  ▄▄▄▄█      █  ▄▄▄▄█  █▄▄▄█      █  █▄▄▄█  ▄▄▄▄█
▄   ▄  ▄▄▄▄▄  ▄▄▄▄▄  ▄▄▄▄▄  ▄▄▄▄▄  ▄▄▄▄▄    ▄    ▄▄▄▄▄  ▄▄▄▄▄
█▄▄▄█  █▄▄▄▄  █▄▄▄▄  ▀   █  █▄▄▄█  █▄▄▄█   ▀█    ▄▄▄▄█   ▄▄▄█
    █  ▄▄▄▄█  █▄▄▄█      █  █▄▄▄█  ▄▄▄▄█   ▄█▄   █▄▄▄▄  ▄▄▄▄█
▄▄▄▄▄  ▄▄▄▄▄  ▄▄▄▄▄    ▄    ▄▄▄▄▄  ▄▄▄▄▄  ▄   ▄  ▄▄▄▄▄  ▄▄▄▄▄
▀   █  █▄▄▄█  █▄▄▄█   ▀█    ▄▄▄▄█   ▄▄▄█  █▄▄▄█  █▄▄▄▄  █▄▄▄▄
    █  █▄▄▄█  ▄▄▄▄█   ▄█▄   █▄▄▄▄  ▄▄▄▄█      █  ▄▄▄▄█  █▄▄▄█
▄▄▄▄▄  ▄▄▄▄▄  ▄   ▄  ▄▄▄▄▄  ▄▄▄▄▄  ▄▄▄▄▄  ▄▄▄▄▄  ▄▄▄▄▄    ▄
▄▄▄▄█   ▄▄▄█  █▄▄▄█  █▄▄▄▄  █▄▄▄▄  ▀   █  █▄▄▄█  █▄▄▄█   ▀█
█▄▄▄▄  ▄▄▄▄█      █  ▄▄▄▄█  █▄▄▄█      █  █▄▄▄█  ▄▄▄▄█   ▄█▄
▄▄▄▄▄  ▄▄▄▄▄  ▄▄▄▄▄  ▄▄▄▄▄  ▄▄▄▄▄    ▄    ▄▄▄▄▄  ▄▄▄▄▄  ▄   ▄
█▄▄▄▄  █▄▄▄▄  ▀   █  █▄▄▄█  █▄▄▄█   ▀█    ▄▄▄▄█   ▄▄▄█  █▄▄▄█
▄▄▄▄█  █▄▄▄█      █  █▄▄▄█  ▄▄▄▄█   ▄█▄   █▄▄▄▄  ▄▄▄▄█      █
▄▄▄▄▄  ▄▄▄▄▄    ▄    ▄▄▄▄▄  ▄▄▄▄▄  ▄   ▄  ▄▄▄▄▄  ▄▄▄▄▄  ▄▄▄▄▄
█▄▄▄█  █▄▄▄█   ▀█    ▄▄▄▄█   ▄▄▄█  █▄▄▄█  █▄▄▄▄  █▄▄▄▄  ▀   █
█▄▄▄█  ▄▄▄▄█   ▄█▄   █▄▄▄▄  ▄▄▄▄█      █  ▄▄▄▄█  █▄▄▄█      █
▄▄▄▄▄  ▄   ▄  ▄▄▄▄▄  ▄▄▄▄▄  ▄▄▄▄▄  ▄▄▄▄▄  ▄▄▄▄▄    ▄    ▄▄▄▄▄
 ▄▄▄█  █▄▄▄█  █▄▄▄▄  █▄▄▄▄  ▀   █  █▄▄▄█  █▄▄▄█   ▀█    ▄▄▄▄█
▄▄▄▄█      █  ▄▄▄▄█  █▄▄▄█      █  █▄▄▄█  ▄▄▄▄█   ▄█▄   █▄▄▄▄
▄▄▄▄▄  ▄▄▄▄▄  ▄▄▄▄▄  ▄▄▄▄▄    ▄    ▄▄▄▄▄  ▄▄▄▄▄  ▄   ▄  ▄▄▄▄▄
█▄▄▄▄  ▀   █  █▄▄▄█  █▄▄▄█   ▀█    ▄▄▄▄█   ▄▄▄█  █▄▄▄█  █▄▄▄▄
█▄▄▄█      █  █▄▄▄█  ▄▄▄▄█   ▄█▄   █▄▄▄▄  ▄▄▄▄█      █  ▄▄▄▄█
▄▄▄▄▄    ▄    ▄▄▄▄▄  ▄▄▄▄▄  ▄   ▄  ▄▄▄▄▄  ▄▄▄▄▄  ▄▄▄▄▄  ▄▄▄▄▄
█▄▄▄█   ▀█    ▄▄▄▄█   ▄▄▄█  █▄▄▄█  █▄▄▄▄  █▄▄▄▄  ▀   █  █▄▄▄█
▄▄▄▄█   ▄█▄   █▄▄▄▄  ▄▄▄▄█      █  ▄▄▄▄█  █▄▄▄█      █  █▄▄▄█
```

Board in small mode:

```
1  2  3  4  5  6  7  8  9
4  5  6  7  8  9  1  2  3
7  8  9  1  2  3  4  5  6
2  3  4  5  6  7  8  9  1
5  6  7  8  9  1  2  3  4
8  9  1  2  3  4  5  6  7
3  4  5  6  7  8  9  1  2
6  7  8  9  1  2  3  4  5
9  1  2  3  4  5  6  7  8
```