use crate::color;

use std::{io::Write, num};

static DIGIT_PATTERNS: [[&str; 3]; 9] = [
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Number(num::NonZero<u8>);

impl Number {
    pub fn new(value: u8) -> Option<Self> {
        (1..=9).contains(&value).then_some(Number(num::NonZero::new(value)?))
    }

    pub fn get(&self) -> u8 {
        self.0.get()
    }

    pub fn get_idx(&self) -> usize {
        self.0.get() as usize - 1
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PencilMask(u16);

impl PencilMask {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn set(self, number: Number) -> Self {
        Self(self.0 | (1 << number.get_idx()))
    }

    pub fn reset(self, number: Number) -> Self {
        Self(self.0 & !(1 << number.get_idx()))
    }

    pub fn contains(&self, number: Number) -> bool {
        self.0 & (1 << number.get_idx()) != 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty { mask: PencilMask },
    Filled { value: Number },
}

impl Cell {
    pub fn new_empty() -> Self {
        Self::Empty { mask: PencilMask::new() }
    }

    pub fn new_with_mask(mask: PencilMask) -> Self {
        Self::Empty { mask }
    }

    pub fn new_filled(value: Number) -> Self {
        Self::Filled { value }
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty { .. })
    }

    pub fn is_filled(&self) -> bool {
        matches!(self, Self::Filled { .. })
    }

    pub fn render(&self, stdout: &mut impl Write, line: usize) -> anyhow::Result<()> {
        match self {
            Self::Empty { mask } => {
                for i in 0..3 {
                    let number = Number::new((line * 3 + i) as u8).expect("`line` should be in 0..3");
                    if mask.contains(number) {
                        write!(stdout, " {}", number.get())?;
                    } else {
                        write!(stdout, "  ")?;
                    }
                }
                write!(stdout, " ")?;
            }
            Self::Filled { value } => {
                write!(stdout, "{}", DIGIT_PATTERNS[value.get_idx()][line])?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    cells: [[Cell; 9]; 9],
    fixed: [[bool; 9]; 9],
}

impl Board {
    pub fn new() -> Self {
        Self { cells: [[Cell::new_empty(); 9]; 9], fixed: [[false; 9]; 9] }
    }

    pub fn get_cell(&self, row: usize, col: usize) -> Cell {
        self.cells[row][col]
    }

    pub fn set_cell(&mut self, row: usize, col: usize, cell: Cell) {
        self.cells[row][col] = cell;
    }

    pub fn count_digits(&self) -> [usize; 9] {
        let mut counts = [0; 9];
        for row in self.cells {
            for cell in row {
                if let Cell::Filled { value } = cell {
                    counts[value.get_idx()] += 1;
                }
            }
        }
        counts
    }

    pub fn render_with_player(&self, stdout: &mut impl Write, player_pos: (usize, usize)) -> anyhow::Result<()> {
        let (pi, pj) = player_pos;
        for i in 0..27 {
            let line = i % 3;
            let i = i / 3;
            for j in 0..9 {
                let color = {
                    if pi == i || pj == j || (pi / 3 == i / 3 && pj / 3 == j / 3) {
                        "\x1b[40m"
                    } else {
                        "\x1b[49m"
                    }
                };
                write!(stdout, "{}", color)?;
                self.cells[i][j].render(stdout, line)?;
            }
            write!(stdout, "\x1b[0m\r\n")?;
        }
        Ok(())
    }
}

pub struct GameState {
    board: Board,
    player_pos: (usize, usize),
    selected_number: Option<Number>,
}

impl GameState {
    pub fn new() -> Self {
        Self { board: Board::new(), player_pos: (0, 0), selected_number: None }
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn get_player_pos(&self) -> (usize, usize) {
        self.player_pos
    }

    pub fn get_selected_number(&self) -> Option<Number> {
        self.selected_number
    }
}
