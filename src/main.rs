use crossterm::{
    cursor,
    event::{self, Event, KeyEvent},
    execute,
    terminal::{self, disable_raw_mode, enable_raw_mode},
};
use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Number(u8);

impl Number {
    fn new(value: u8) -> Option<Self> {
        (0..9).contains(&value).then_some(Self(value))
    }

    fn from_modulo(value: u8) -> Self {
        Self(value % 9)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PencilMask(u16);

impl PencilMask {
    fn new() -> Self {
        Self(0)
    }

    fn set(self, number: Number) -> Self {
        Self(self.0 | (1 << number.0))
    }

    fn reset(self, number: Number) -> Self {
        Self(self.0 & !(1 << number.0))
    }

    fn contains(&self, number: Number) -> bool {
        self.0 & (1 << number.0) != 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty { mask: PencilMask },
    Filled { value: Number },
}

impl Cell {
    fn new_empty() -> Self {
        Self::Empty { mask: PencilMask::new() }
    }

    fn new_with_mask(mask: PencilMask) -> Self {
        Self::Empty { mask }
    }

    fn new_filled(value: Number) -> Self {
        Self::Filled { value }
    }

    fn is_empty(&self) -> bool {
        matches!(self, Self::Empty { .. })
    }

    fn is_filled(&self) -> bool {
        matches!(self, Self::Filled { .. })
    }

    fn render(&self, line: usize) {
        match self {
            Self::Empty { mask } => {
                for i in 0..3 {
                    let number = Number::new((line * 3 + i) as u8).unwrap();
                    if mask.contains(number) {
                        print!(" {}", number.0 + 1);
                    } else {
                        print!("  ");
                    }
                }
                print!(" ");
            }
            Self::Filled { value } => {
                print!("{}", DIGIT_PATTERNS[value.0 as usize][line]);
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Board {
    cells: [[Cell; 9]; 9],
}

impl Board {
    fn new() -> Self {
        Self { cells: [[Cell::new_empty(); 9]; 9] }
    }

    fn get_cell(&self, row: usize, col: usize) -> Cell {
        self.cells[row][col]
    }

    fn set_cell(&mut self, row: usize, col: usize, cell: Cell) {
        self.cells[row][col] = cell;
    }

    fn render(&self, player_pos: Option<(usize, usize)>) {
        let (pi, pj) = player_pos.unwrap_or((9, 9));
        for i in 0..27 {
            let line = i % 3;
            let i = i / 3;
            for j in 0..9 {
                let color = {
                    if pi == i && pj == j {
                        "\x1b[48;5;240m"
                    } else if pi == i || pj == j || (pi / 3 == i / 3 && pj / 3 == j / 3) {
                        "\x1b[48;5;238m"
                    } else if (i / 3 + j / 3) % 2 == 0 {
                        "\x1b[48;5;234m"
                    } else {
                        "\x1b[48;5;235m"
                    }
                };
                print!("{}", color);
                self.cells[i][j].render(line);
            }
            println!("\x1b[0m");
        }
    }
}

const DIGIT_PATTERNS: [[&str; 3]; 9] = [
    ["   ▄   ", "  ▀█   ", "  ▄█▄  "],
    [" ▄▄▄▄▄ ", " ▄▄▄▄█ ", " █▄▄▄▄ "],
    [" ▄▄▄▄▄ ", "  ▄▄▄█ ", " ▄▄▄▄█ "],
    [" ▄   ▄ ", " █▄▄▄█ ", "     █ "],
    [" ▄▄▄▄▄ ", " █▄▄▄▄ ", " ▄▄▄▄█ "],
    [" ▄▄▄▄▄ ", " █▄▄▄▄ ", " █▄▄▄█ "],
    [" ▄▄▄▄▄ ", " ▀   █ ", "     █ "],
    [" ▄▄▄▄▄ ", " █▄▄▄█ ", " █▄▄▄█ "],
    [" ▄▄▄▄▄ ", " █▄▄▄█ ", " ▄▄▄▄█ "],
];

use clap::Parser;

/// Sudoku game in terminal
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Play on smaller board
    #[arg(short, long)]
    small: bool,
}

fn main() -> anyhow::Result<()> {
    let _args = Args::parse();

    let (w, h) = terminal::size()?;
    if w < 27 || h < 27 {
        anyhow::bail!("Terminal size must be at least 27x27");
    }

    // enable_raw_mode()?;

    let mut board = Board::new();

    board.set_cell(0, 0, Cell::new_filled(Number(0)));
    board.set_cell(0, 1, Cell::new_filled(Number(1)));
    board.set_cell(0, 2, Cell::new_filled(Number(2)));
    board.set_cell(1, 0, Cell::new_filled(Number(3)));
    board.set_cell(1, 1, Cell::new_filled(Number(4)));
    board.set_cell(1, 2, Cell::new_filled(Number(5)));
    board.set_cell(2, 0, Cell::new_filled(Number(6)));
    board.set_cell(2, 1, Cell::new_filled(Number(7)));
    board.set_cell(2, 2, Cell::new_filled(Number(8)));

    board.set_cell(1, 7, Cell::new_with_mask(PencilMask(0b101010101)));

    board.set_cell(4, 4, Cell::new_with_mask(PencilMask(0b111101111)));

    board.set_cell(7, 1, Cell::new_with_mask(PencilMask(0b010101010)));

    board.set_cell(7, 7, Cell::new_with_mask(PencilMask(0b111111111)));

    board.render(Some((4, 7)));

    // loop {
    //     let event = event::read()?;
    //     let Event::Key(KeyEvent { code, modifiers, .. }) = event else {
    //         continue
    //     };

    //     print!("{code}: {modifiers}\r\n");
    //     if code == event::KeyCode::Char('q') {
    //         break;
    //     }
    // }

    // disable_raw_mode()?;
    Ok(())
}
