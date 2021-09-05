use super::StrategyResult;
use crate::{
    solver::House,
    Sudoku, Pos,
};
use std::{
    cmp::{min, max},
    ops::RangeInclusive,
};

pub(crate) fn naked_subset(sudoku: &Sudoku, subset_sizes: &RangeInclusive<usize>) -> Option<StrategyResult> {
    for subset_size in max(2, *subset_sizes.start())..=min(4, *subset_sizes.end()) {
        let res = match subset_size {
            2 => naked_pair(sudoku),
            3 => naked_triple(sudoku),
            4 => naked_quad(sudoku),
            _ => unreachable!()
        };
        if res.is_some() { return res }
    }
    None
}

fn naked_pair(sudoku: &Sudoku) -> Option<StrategyResult> {
    for house in House::iter() {
        let positions: Vec<Pos> = house.members_iter().collect();
        for i1 in 0..positions.len() - 1 {
            let candidates1 = sudoku.get_candidates_by_pos(positions[i1]);
            if candidates1.len() != 2 { continue }

            for i2 in i1+1 .. positions.len() {
                let candidates2 = sudoku.get_candidates_by_pos(positions[i2]);
                if candidates2.len() != 2 { continue }

                let candidates = candidates1 | candidates2;
                if candidates.len() == 2 {
                    let mut exclusions = Vec::new();
                    for j in 0..positions.len() {
                        if j == i1 || j == i2 { continue }
                        for val in (candidates & sudoku.get_candidates_by_pos(positions[j])).iter() {
                            exclusions.push((positions[j], val));
                        }
                    }
                    if !exclusions.is_empty() {
                        return Some(StrategyResult::NakedSubset {
                            exclusions,
                            positions: vec![positions[i1], positions[i2]],
                            values: candidates.iter().collect(),
                            house
                        });
                    }
                }
            }
        }
    }
    None
}

fn naked_triple(sudoku: &Sudoku) -> Option<StrategyResult> {
    // FIXME
    None
}

fn naked_quad(sudoku: &Sudoku) -> Option<StrategyResult> {
    // FIXME
    None
}

