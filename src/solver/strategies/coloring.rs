use super::StrategyResult;
use crate::{
    solver::{House, PosBitSet, PosIndexedSlice, ValueIndexedSlice},
    Pos, Sudoku, Value,
};
use std::{
    collections::VecDeque,
    cmp::{min, max},
    iter::FromIterator,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Color(usize);

impl Color {
    const NONE: Color = Color(usize::MAX);

    #[inline]
    fn is_pair(self, other: Color) -> bool {
        (self.0 ^ 1) == other.0
    }

    #[inline]
    fn get_pair(self) -> Color {
        Color(self.0 ^ 1)
    }

    #[inline]
    fn get_base(self) -> Color {
        Color(self.0 & !1)
    }

    #[inline]
    fn as_usize(self) -> usize {
        self.0
    }
}


pub(crate) struct Coloring {
    by_pos: PosIndexedSlice<Color>,
    by_color: Vec<PosBitSet>
}

impl Coloring {
    #[inline]
    fn new() -> Self {
        Self {
            by_pos: PosIndexedSlice::from_slice([Color::NONE; Pos::N]),
            by_color: Vec::new()
        }
    }

    #[inline]
    fn next_colors(&self) -> [Color; 2] {
        let len = self.by_color.len();
        [Color(len), Color(len + 1)]
    }

    // Iterate over each color, skipping the paired colors
    #[inline]
    fn unique_color_iter(&self) -> impl Iterator<Item = Color> {
        (0..self.by_color.len()).step_by(2).map(|idx| Color(idx))
    }

    fn add_conjugate_pair(&mut self, pos1: Pos, pos2: Pos) {
        match [self.by_pos[pos1], self.by_pos[pos2]] {
            [Color::NONE, Color::NONE] => {
                let [color1, color2] = self.next_colors();
                self.by_pos[pos1] = color1;
                self.by_pos[pos2] = color2;
                self.by_color.push(PosBitSet::from_iter([pos1]));
                self.by_color.push(PosBitSet::from_iter([pos2]));
            },
            [Color::NONE, color2] => {
                let color1 = color2.get_pair();
                self.by_pos[pos1] = color1;
                self.by_color[color1.as_usize()].insert(pos1);
            },
            [color1, Color::NONE] => {
                let color2 = color1.get_pair();
                self.by_pos[pos2] = color2;
                self.by_color[color2.as_usize()].insert(pos2);
            },
            [color1, color2] if color1.is_pair(color2) => (), // Already in correct spot
            [color1, color2] => {
                let color_a = min(color1, color2);
                let color_b = max(color1, color2);
                self.shift_color_pairs(color_b, color_a.get_pair());

                let lastcolor = Color(self.by_color.len() - 2);
                if color_b.get_base() < lastcolor {
                    self.shift_color_pairs(lastcolor, color_b.get_base());
                }
                self.by_color.truncate(self.by_color.len() - 2);
            }
        }
    }

    fn shift_color_pairs(&mut self, old_color: Color, new_color: Color) {
        for pos in self.by_color[old_color.as_usize()].iter() {
            self.by_color[new_color.as_usize()].insert(pos);
            self.by_pos[pos] = new_color;
        }
        for pos in self.by_color[old_color.get_pair().as_usize()].iter() {
            self.by_color[new_color.get_pair().as_usize()].insert(pos);
            self.by_pos[pos] = new_color.get_pair();
        }
        self.by_color[old_color.as_usize()] = PosBitSet::NONE;
        self.by_color[old_color.get_pair().as_usize()] = PosBitSet::NONE;
    }
}

fn build_coloring(positions: PosBitSet) -> Coloring {
    let mut ret = Coloring::new();
    for house in House::iter() {
        let candidates = positions & house.members_bitset();
        if candidates.len() != 2 { continue }
        let mut candidates_iter = candidates.iter();
        let pos1 = candidates_iter.next().unwrap();
        let pos2 = candidates_iter.next().unwrap();

        ret.add_conjugate_pair(pos1, pos2);
    }
    ret
}

// Two cells with the same color (and parity) see each other, implies all of them must be false
// since there is a contradiction
fn color_wrap(coloring: &Coloring, val: Value) -> Option<StrategyResult> {
    for color in coloring.unique_color_iter() {
        for color in [color, color.get_pair()] {
            let positions = coloring.by_color[color.as_usize()];
            for pos in positions.iter() {
                if !(pos.neighbors_bitset() & positions).is_empty() {
                    return Some(StrategyResult::SimpleColor {
                        excluded_candidates: positions.iter().map(|pos| (pos, val)).collect(),
                        value: val,
                        color_positions: [
                            positions.iter().collect(),
                            coloring.by_color[color.get_pair().as_usize()].iter().collect(),
                        ],
                        color_wrap: true,
                    });
                }
            }
        }
    }
    None
}

// Any candidates which see both pairs of a color are false
fn color_trap(sudoku: &Sudoku, coloring: &Coloring, val: Value) -> Option<StrategyResult> {
    for color in coloring.unique_color_iter() {
        let neighbors1 = coloring.by_color[color.as_usize()].iter()
            .fold(PosBitSet::NONE, |neighbors, pos| neighbors | pos.neighbors_bitset());
        let neighbors2 = coloring.by_color[color.get_pair().as_usize()].iter()
            .fold(PosBitSet::NONE, |neighbors, pos| neighbors | pos.neighbors_bitset());

        let excluded_positions = neighbors1 & neighbors2 & sudoku.get_candidates_by_value(val);
        if !excluded_positions.is_empty() {
            return Some(StrategyResult::SimpleColor {
                excluded_candidates: excluded_positions.iter().map(|pos| (pos, val)).collect(),
                value: val,
                color_positions: [
                    coloring.by_color[color.as_usize()].iter().collect(),
                    coloring.by_color[color.get_pair().as_usize()].iter().collect(),
                ],
                color_wrap: false,
            });
        }
    }
    None
}

pub(crate) fn simple_color(sudoku: &Sudoku, colorings: &mut ValueIndexedSlice<Option<Coloring>>) -> Option<StrategyResult> {
    for val in Value::iter() {
        if sudoku.get_candidates_by_value(val).is_empty() { continue }
        if colorings[val].is_none() {
            colorings[val] = Some(build_coloring(sudoku.get_candidates_by_value(val)));
        }
        let coloring = colorings[val].as_ref().unwrap();

        if let res@Some(_) = color_wrap(coloring, val) { return res }
        if let res@Some(_) = color_trap(sudoku, coloring, val) { return res }
    }
    None
}

fn build_color_neighbors(coloring: &Coloring) -> Vec<Vec<Color>> {
    let mut ret = vec![Vec::new(); coloring.by_color.len()];
    for color in coloring.unique_color_iter() {
        for color in [color, color.get_pair()] {
            let neighbors = coloring.by_color[color.as_usize()].iter()
                .fold(PosBitSet::NONE, |neighbors, pos| neighbors | pos.neighbors_bitset());
            let mut neighbor_colors: Vec<Color> = neighbors.iter()
                .map(|pos| coloring.by_pos[pos])
                .filter(|&neigh_color| neigh_color != Color::NONE && !neigh_color.is_pair(color))
                .collect();
            neighbor_colors.sort_unstable();
            neighbor_colors.dedup();
            ret[color.as_usize()].append(&mut neighbor_colors);
        }
    }
    ret
}

fn color_wing(sudoku: &Sudoku, coloring: &Coloring, val: Value, start_color: Color, end_color: Color, color_path: &Vec<Color>)
    -> Option<StrategyResult>
{
    let neighbors1 = coloring.by_color[start_color.as_usize()].iter()
        .fold(PosBitSet::NONE, |neighbors, pos| neighbors | pos.neighbors_bitset());
    let neighbors2 = coloring.by_color[end_color.as_usize()].iter()
        .fold(PosBitSet::NONE, |neighbors, pos| neighbors | pos.neighbors_bitset());

    let mut excluded_positions = neighbors1 & neighbors2 & sudoku.get_candidates_by_value(val);
    if !excluded_positions.is_empty() {
        // If any excluded positions are colored, all with that color are also excluded
        for pos in excluded_positions.iter() {
            let color = coloring.by_pos[pos];
            if color != Color::NONE {
                excluded_positions |= coloring.by_color[color.as_usize()];
            }
        }
        return Some(StrategyResult::MultiColor {
            excluded_candidates: excluded_positions.iter().map(|pos| (pos, val)).collect(),
            value: val,
            color_positions: color_path.iter().map(|color| [
                coloring.by_color[color.as_usize()].iter().collect(),
                coloring.by_color[color.get_pair().as_usize()].iter().collect(),
            ]).collect()
        });
    }
    None
}

// Finds color wing of any length up to max_color_pairs
pub(crate) fn multi_color(sudoku: &Sudoku, max_color_pairs: usize, colorings: &mut ValueIndexedSlice<Option<Coloring>>) -> Option<StrategyResult> {
    for val in Value::iter() {
        if sudoku.get_candidates_by_value(val).is_empty() { continue }
        if colorings[val].is_none() {
            colorings[val] = Some(build_coloring(sudoku.get_candidates_by_value(val)));
        }
        let coloring = colorings[val].as_ref().unwrap();

        let mut queue = VecDeque::new();
        let color_neighbors = build_color_neighbors(coloring);
        for color in coloring.unique_color_iter() {
            queue.push_back(vec![color]);
            queue.push_back(vec![color.get_pair()]);
        }

        while let Some(color_path) = queue.pop_front() {
            let start_color = color_path[0];
            let end_color = color_path[color_path.len() - 1].get_pair();

            // len=1 is handled as simple_color
            if color_path.len() > 1 {
                if let ret@Some(_) = color_wing(sudoku, coloring, val, start_color, end_color, &color_path) {
                    return ret
                }
            }
            if color_path.len() >= max_color_pairs { continue }

            for &neigh_color in color_neighbors[end_color.as_usize()].iter() {
                if neigh_color < start_color { continue } // We'll find this already in the other direction
                let mut new_color_path = color_path.clone();
                new_color_path.push(neigh_color);
                queue.push_back(new_color_path);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Pos;

    fn check_simple_color_example(sudoku_line: &str, expected_res: Option<StrategyResult>) {
        let sudoku = Sudoku::from_line(sudoku_line).unwrap();
        const NONE: Option<Coloring> = None;
        let mut colorings = ValueIndexedSlice::from_slice([NONE; Value::N]);
        let res = simple_color(&sudoku, &mut colorings);
        assert_eq!(res, expected_res);
    }

    #[test]
    fn test_color_wrap_example1() {
        check_simple_color_example(
            "97864253161.753.8.3.589167.453169827297.8516.186.27...56127439883.9167..7.9538.16",
            Some(StrategyResult::SimpleColor {
                excluded_candidates: vec![
                    (Pos::new(11), Value::new(1)), (Pos::new(15), Value::new(1)), (Pos::new(26), Value::new(1)),
                    (Pos::new(71), Value::new(1)), (Pos::new(73), Value::new(1))
                ],
                value: Value::new(1),
                color_positions: [
                    vec![Pos::new(11), Pos::new(15), Pos::new(26), Pos::new(71), Pos::new(73)],
                    vec![Pos::new(19), Pos::new(65), Pos::new(78)]
                ],
                color_wrap: true
            }));
    }

    #[test]
    fn test_color_trap_example1() {
        check_simple_color_example(
            ".973.84.64..69.387683...95.9.683.57473...68.9.489..63.37428916586...4293..9.63748",
            Some(StrategyResult::SimpleColor {
                excluded_candidates: vec![(Pos::new(45), Value::new(4))],
                value: Value::new(4),
                color_positions: [
                    vec![Pos::new(0), Pos::new(14)],
                    vec![Pos::new(4), Pos::new(50)]
                ],
                color_wrap: false
            }));
    }

    fn check_multi_color_example(max_color_pairs: usize, sudoku_line: &str, expected_res: Option<StrategyResult>) {
        let sudoku = Sudoku::from_line(sudoku_line).unwrap();
        const NONE: Option<Coloring> = None;
        let mut colorings = ValueIndexedSlice::from_slice([NONE; Value::N]);
        let res = multi_color(&sudoku, max_color_pairs, &mut colorings);
        assert_eq!(res, expected_res);
    }

    #[test]
    fn test_multi_color_example1() {
        check_multi_color_example(usize::MAX,
            "751496328.24..19.7...27.4...7...2.4.182647593.4.91.7.24.576.2.929...467...7.29..4",
            Some(StrategyResult::MultiColor {
                excluded_candidates: vec![(Pos::new(30), Value::new(4))],
                value: Value::new(4),
                color_positions: vec![
                    [
                        vec![Pos::new(27), Pos::new(50)],
                        vec![Pos::new(23), Pos::new(45)],
                    ], [
                        vec![Pos::new(26), Pos::new(79)],
                        vec![Pos::new(71), Pos::new(75)],
                    ],
                ],
            }))
    }

    #[test]
    fn test_multi_color_example2() {
        // All of the color is excluded
        check_multi_color_example(usize::MAX,
            "157248639283697...6..531728...48..13..8123..731.75.8.....374.8.8.19623.5.3.815..6",
            Some(StrategyResult::MultiColor {
                excluded_candidates: vec![
                    (Pos::new(33), Value::new(1)), (Pos::new(47), Value::new(1)), (Pos::new(62), Value::new(1))
                ],
                value: Value::new(1),
                color_positions: vec![
                    [
                        vec![Pos::new(53)],
                        vec![Pos::new(33), Pos::new(47), Pos::new(62)]
                    ], [
                        vec![Pos::new(74)],
                        vec![Pos::new(78)],
                    ],
                ],
            }));
    }

    #[test]
    fn test_multi_color_example3() {
        let sudoku_line = "71.....69.924.7.1...5....2727...5.4.1.387.295.5..2..71531...784427581936986743152";
        check_multi_color_example(usize::MAX, sudoku_line,
            Some(StrategyResult::MultiColor {
                excluded_candidates: vec![(Pos::new(21), Value::new(2))],
                value: Value::new(2),
                color_positions: vec![
                    [
                        vec![Pos::new(48)],
                        vec![Pos::new(51)],
                    ], [
                        vec![Pos::new(35)],
                        vec![Pos::new(17)],
                    ], [
                        vec![Pos::new(9)],
                        vec![Pos::new(18)],
                    ],
                ],
            }));

        // Max chain length is respected
        check_multi_color_example(2, sudoku_line, None);
    }
}
