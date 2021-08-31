#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Pos(u8);

impl Pos {
    pub const MIN: Pos = Pos(0);
    pub const MAX: Pos = Pos(80);

    #[inline]
    pub fn new(idx: usize) -> Self {
        if idx >= 81 { panic!("Index out of bounds") }
        Self(idx as u8)
    }

    #[inline]
    pub const unsafe fn new_unchecked(idx: usize) -> Self {
        Self(idx as u8)
    }

    #[inline]
    pub fn row_col(row: u8, col: u8) -> Self {
        if row >= 9 { panic!("Row out of bounds") }
        if col >= 9 { panic!("Col out of bounds") }
        Self(row * 9 + col)
    }

    #[inline]
    pub const fn row(&self) -> u8 {
        self.0 / 9
    }

    #[inline]
    pub const fn col(&self) -> u8 {
        self.0 % 9
    }

    /// 
    /// Blocks are numbered in the same way as positions, e.g.
    /// ```text
    /// 0 1 2
    /// 3 4 5
    /// 6 7 8
    /// ```
    #[inline]
    pub const fn block(&self) -> u8 {
        (self.row() / 3) * 3 + (self.col() / 3) // TODO - better as a lookup table?
    }

    #[inline]
    pub const fn idx(&self) -> usize {
        self.0 as usize
    }

    #[inline]
    pub fn iter() -> Iter {
        Iter(Pos::MIN.0)
    }
}

pub struct Iter(u8);

impl Iterator for Iter {
    type Item = Pos;
    #[inline]
    fn next(&mut self) -> Option<Pos> {
        if self.0 > Pos::MAX.0 {
            None
        } else {
            let ret = Some(Pos(self.0));
            self.0 += 1;
            ret
        }
    }
}
