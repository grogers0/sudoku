use crate::{
    bitset::{BitSet9, BitSet81},
    pos::Pos,
    neighbors::{neighbor_positions, neighbor_bitset},
};
use std::{
    cmp::max,
    fmt,
    num::NonZeroU8,
};

#[derive(Clone, PartialEq, Eq)]
pub struct Sudoku {
    values: [Option<NonZeroU8>; 81],
    candidates_by_pos: [BitSet9; 81],
    candidates_by_value: [BitSet81; 9]
}

#[derive(Debug)]
pub enum SudokuParseError {
    TooMuchInput,
    TooLittleInput,
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

    #[inline]
    pub fn get_value(&self, pos: Pos) -> Option<NonZeroU8> {
        self.values[pos.idx()]
    }

    #[inline]
    pub fn get_candidates_by_pos(&self, pos: Pos) -> BitSet9 {
        self.candidates_by_pos[pos.idx()]
    }

    #[inline]
    pub fn get_candidates_by_value(&self, value: NonZeroU8) -> BitSet81 {
        self.candidates_by_value[value.get() as usize - 1]
    }

    #[inline]
    pub fn progress_possible(&self) -> bool {
        let mut candidates = BitSet81::NONE;
        for value_idx in 0..9 {
            candidates |= self.candidates_by_value[value_idx];
        }
        candidates != BitSet81::NONE
    }

    pub fn is_solved(&self) -> bool {
        if self.progress_possible() {
            return false
        }

        for pos in Pos::iter() {
            let val = self.values[pos.idx()];
            for pos2 in neighbor_positions(pos) {
                if val == self.values[pos2.idx()] {
                    return false
                }
            }
        }
        true
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

        self.check_consistency();
    }

    pub fn remove_candidate(&mut self, pos: Pos, value: NonZeroU8) {
        if value.get() > 9 { panic!("Value out of bounds") }
        if self.candidates_by_pos[pos.idx()].get((value.get() - 1) as usize) { panic!("Cell doesn't contain this candidate") }

        self.candidates_by_pos[pos.idx()].clear((value.get() - 1) as usize);
        self.candidates_by_value[(value.get() - 1) as usize].clear(pos.idx());

        self.check_consistency();
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
    pub fn from_line(line: &str) -> Result<Sudoku, SudokuParseError> {
        let mut sudoku = Sudoku::new();
        let mut got_enough = false;
        for (i, ch) in line.chars().enumerate() {
            if got_enough { return Err(SudokuParseError::TooMuchInput) }
            match ch {
                '.' | '_' | '0' => (),
                '1' ..= '9' => sudoku.set_value(Pos::new(i), NonZeroU8::new(ch.to_digit(10).unwrap() as u8).unwrap()),
                _ => return Err(SudokuParseError::InvalidChar(ch))
            }
            if i == 80 { got_enough = true }
        }
        if !got_enough { return Err(SudokuParseError::TooLittleInput) }
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

    /// Parses a sudoku of the form:
    /// 
    /// ```text
    /// 1236  7      139  | 169   5      4     | 8     129    129
    /// 12    4      159  | 7     189    28    | 3     6      1259
    /// 1236  13569  8    | 169   1369   236   | 2579  12579  4
    /// ------------------+--------------------+-------------------
    /// 7     38     4    | 589   389    1     | 259   2359   6
    /// 9     136    2    | 456   3467   3567  | 57    1357   8
    /// 5     1368   13   | 2     36789  3678  | 4     1379   1379
    /// ------------------+--------------------+-------------------
    /// 348   3589   359  | 4568  4678   5678  | 1     23579  23579
    /// 134   2      6    | 145   147    9     | 57    8      357
    /// 18    1589   7    | 3     2      58    | 6     4      59
    /// ```
    pub fn from_pencilmarks(s: &str) -> Result<Sudoku, SudokuParseError> {
        struct ParseState<PosIter : Iterator<Item=Pos>> {
            has_whitespace: bool,
            pos_iter: PosIter,
            candidates: BitSet9,
            values_to_set: Vec<(Pos, NonZeroU8)>
        }
        let mut state = ParseState {
            has_whitespace: false,
            pos_iter: Pos::iter(),
            candidates: BitSet9::NONE,
            values_to_set: Vec::new()
        };
        let mut sudoku = Sudoku {
            values: [None; 81],
            candidates_by_pos: [BitSet9::NONE; 81],
            candidates_by_value: [BitSet81::NONE; 9]
        };

        fn process_prev_cell<PosIter : Iterator<Item=Pos>>(state: &mut ParseState<PosIter>,
            sudoku: &mut Sudoku) -> Result<(), SudokuParseError> {
            let pos = match state.pos_iter.next() {
                Some(p) => p,
                None => return Err(SudokuParseError::TooMuchInput)
            };
            if state.candidates.count_ones() == 1 {
                state.values_to_set.push((pos,
                        NonZeroU8::new(state.candidates.iter().next().unwrap() as u8 + 1).unwrap()));
            } else {
                for value_idx in state.candidates.iter() {
                    sudoku.candidates_by_value[value_idx].set(pos.idx());
                }
                sudoku.candidates_by_pos[pos.idx()] = state.candidates;
            };
            Ok(())
        }

        for ch in s.trim().chars() {
            match ch {
                '-' | '+' | '|' => state.has_whitespace = true,
                x if x.is_whitespace() => state.has_whitespace = true,
                '1' ..= '9' => {
                    if state.has_whitespace {
                        process_prev_cell(&mut state, &mut sudoku)?;
                        state.has_whitespace = false;
                        state.candidates = BitSet9::NONE;
                    }

                    state.candidates.set(ch.to_digit(10).unwrap() as usize - 1);
                },
                _ => return Err(SudokuParseError::InvalidChar(ch))
            }
        }
        process_prev_cell(&mut state, &mut sudoku)?;
        if state.pos_iter.next().is_some() {
            return Err(SudokuParseError::TooLittleInput)
        }
        // At the end so if the pencilmarks had superfluous candidates we would eliminate them,
        // instead of having to treat them as naked singles or something
        for (pos, value) in state.values_to_set {
            sudoku.set_value(pos, value);
        }
        sudoku.check_consistency();
        Ok(sudoku)
    }

    /// Outputs a sudoku in pencilmarks format (see [`Sudoku::from_pencilmarks`]).
    pub fn to_pencilmarks(&self) -> String {
        let mut widths = [1; 9]; // Per col
        for pos in Pos::iter() {
            widths[pos.col() as usize] = max(
                widths[pos.col() as usize],
                self.candidates_by_pos[pos.idx()].count_ones());
        }

        let mut s = String::new();
        for pos in Pos::iter() {
            let mut written_chars = 0;
            if let Some(value) = self.values[pos.idx()] {
                s.push(char::from_digit(value.get() as u32, 10).unwrap());
                written_chars = 1;
            } else {
                for value_idx in self.candidates_by_pos[pos.idx()].iter() {
                    s.push(char::from_digit((value_idx + 1) as u32, 10).unwrap());
                    written_chars += 1;
                }
            }
            while written_chars < widths[pos.col() as usize] {
                s.push(' ');
                written_chars += 1;
            }

            if pos.col() % 3 == 2 {
                if pos.col() == 8 {
                    if pos.row() != 8 {
                        s.push('\n');
                    }
                    if pos.row() == 2 || pos.row() == 5 {
                        for _ in 0..(widths[0..3].iter().sum::<usize>() + 5) { s.push('-') }
                        s.push('+');
                        for _ in 0..(widths[3..6].iter().sum::<usize>() + 6) { s.push('-') }
                        s.push('+');
                        for _ in 0..(widths[6..9].iter().sum::<usize>() + 5) { s.push('-') }
                        s.push('\n');
                    }
                } else {
                    s.push_str(" | ");
                }
            } else {
                s.push_str("  ");
            }
        }
        s
    }

    #[cfg(debug_assertions)]
    fn check_consistency(&self) {
        for pos in Pos::iter() {
            if let Some(_) = self.values[pos.idx()] {
                if self.candidates_by_pos[pos.idx()] != BitSet9::NONE {
                    panic!("Cell with value set but has candidates");
                }
            }

            for val_idx in 0..9 {
                if self.candidates_by_pos[pos.idx()].get(val_idx) != self.candidates_by_value[val_idx].get(pos.idx()) {
                    panic!("Candidates by pos don't match candidates by value");
                }
            }
        }
    }

    #[inline]
    #[cfg(not(debug_assertions))]
    fn check_consistency(&self) { }
}

impl fmt::Debug for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str(&self.to_pencilmarks())
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

    #[test]
    fn test_from_to_pencilmarks() {
        let line = "4...3.......6..8..........1....5..9..8....6...7.2........1.27..5.3....4.9........";
        let expected_pencilmarks = "4      12569  1256789 | 5789   3      15789  | 259    2567    25679 
1237   12359  12579   | 6      12479  14579  | 8      2357    234579
23678  23569  256789  | 45789  24789  45789  | 23459  23567   1     
----------------------+----------------------+----------------------
1236   12346  1246    | 3478   5      134678 | 1234   9       23478 
123    8      12459   | 3479   1479   13479  | 6      12357   23457 
136    7      14569   | 2      14689  134689 | 1345   1358    3458  
----------------------+----------------------+----------------------
68     46     468     | 1      4689   2      | 7      3568    35689 
5      126    3       | 789    6789   6789   | 129    4       2689  
9      1246   124678  | 34578  4678   345678 | 1235   123568  23568 ";

        let sudoku = Sudoku::from_line(line).unwrap();
        let pencilmarks = sudoku.to_pencilmarks();
        assert_eq!(&pencilmarks, expected_pencilmarks);
        let sudoku2 = Sudoku::from_pencilmarks(&pencilmarks).unwrap();
        assert_eq!(sudoku, sudoku2);
    }
}
