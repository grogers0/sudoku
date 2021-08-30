use crate::{
    bitset::{BitSet9, BitSet81},
    pos::Pos,
    neighbors::{neighbor_positions, neighbor_bitset},
};
use std::num::NonZeroU8;

pub struct Sudoku {
    values: [Option<NonZeroU8>; 81],
    candidates_by_pos: [BitSet9; 81],
    candidates_by_value: [BitSet81; 9]
}

#[derive(Debug)]
pub enum LineParseError {
    TooManyChars,
    TooFewChars,
    InvalidChar(char)
}

impl Sudoku {
    pub fn new() -> Self {
        Self {
            values: [None; 81],
            candidates_by_pos: [BitSet9::ALL; 81],
            candidates_by_value: [BitSet81::ALL; 9]
        }
    }

    pub fn set_value(&mut self, pos: Pos, value: NonZeroU8) {
        if value.get() > 9 { panic!("Value out of bounds") }
        if self.values[pos.idx()].is_some() { panic!("Cell already has value present") }

        self.values[pos.idx()] = Some(value);

        self.candidates_by_pos[pos.idx()] = BitSet9::NONE;
        for val2_idx in 0..9 {
            self.candidates_by_value[val2_idx].clear(pos.idx());
        }

        for pos2 in neighbor_positions(pos) {
            self.candidates_by_pos[pos2.idx()].clear((value.get() - 1) as usize);
        }
        self.candidates_by_value[(value.get() - 1) as usize] &= !neighbor_bitset(pos);
    }

    /// Parses a sudoku of the form:
    /// 
    /// ```text
    /// 4...3.......6..8..........1....5..9..8....6...7.2........1.27..5.3....4.9........
    /// ```
    ///
    /// into the sudoku:
    ///
    /// ```text
    /// +---+---+---+
    /// |4  | 3 |   |
    /// |   |6  |8  |
    /// |   |   |  1|
    /// +---+---+---+
    /// |   | 5 | 9 |
    /// | 8 |   |6  |
    /// | 7 |2  |   |
    /// +---+---+---+
    /// |   |1 2|7  |
    /// |5 3|   | 4 |
    /// |9  |   |   |
    /// +---+---+---+
    /// ```
    ///
    /// For empty cells, `'.'`, `'_'`, or `'0'` are allowed.
    pub fn from_line(line: &str) -> Result<Sudoku, LineParseError> {
        let mut sudoku = Sudoku::new();
        let mut got_enough = false;
        for (i, ch) in line.chars().enumerate() {
            if got_enough { return Err(LineParseError::TooManyChars) }
            match ch {
                '.' | '_' | '0' => (),
                '1' ..= '9' => sudoku.set_value(Pos::new(i), NonZeroU8::new(ch.to_digit(10).unwrap() as u8).unwrap()),
                _ => return Err(LineParseError::InvalidChar(ch))
            }
            if i == 80 { got_enough = true }
        }
        if !got_enough { return Err(LineParseError::TooFewChars) }
        Ok(sudoku)
    }

    /// Outputs a sudoku in line format (see [`Sudoku::from_line`]).
    pub fn to_line(&self) -> String {
        let mut line = String::with_capacity(81);
        for value in self.values {
            match value {
                Some(v) => line.push(char::from_digit(v.get() as u32, 10).unwrap()),
                None => line.push('.')
            }
        }
        line
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_to_line() {
        let line = "4...3.......6..8..........1....5..9..8....6...7.2........1.27..5.3....4.9........";
        let sudoku = Sudoku::from_line(line).unwrap();
        let line2 = sudoku.to_line();
        assert_eq!(line, &line2);
    }
}
