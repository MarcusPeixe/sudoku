#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Number(std::num::NonZero<u8>);

impl Number {
    pub fn new(value: u8) -> Option<Self> {
        (1..=9)
            .contains(&value)
            .then_some(Number(std::num::NonZero::new(value)?))
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
        Self::Empty {
            mask: PencilMask::new(),
        }
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    pub cells: [[Cell; 9]; 9],
}

impl Board {
    pub fn new() -> Self {
        Self {
            cells: [[Cell::new_empty(); 9]; 9],
        }
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

    pub fn get_invalid(&self) -> [[[bool; 9]; 9]; 9] {
        let mut invalid = [[[false; 9]; 9]; 9];
        #[allow(clippy::needless_range_loop)]
        for bi in 0..9 {
            for bj in 0..9 {
                if let Cell::Filled { value } = self.cells[bi][bj] {
                    let idx = value.get_idx();
                    for i in 0..9 {
                        for j in 0..9 {
                            if (i != bi || j != bj)
                                && (i == bi || j == bj || (i / 3 == bi / 3 && j / 3 == bj / 3))
                            {
                                invalid[i][j][idx] = true;
                            }
                        }
                    }
                }
            }
        }
        invalid
    }
}

pub struct GameState {
    pub board: Board,
    pub fixed: [[bool; 9]; 9],
    pub invalid: [[[bool; 9]; 9]; 9],
    pub player_pos: (usize, usize),
    pub selected_number: Option<Number>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            fixed: [[false; 9]; 9],
            invalid: [[[false; 9]; 9]; 9],
            player_pos: (0, 0),
            selected_number: None,
        }
    }

    pub fn update_invalid(&mut self) {
        self.invalid = self.board.get_invalid();
    }
}
