use super::StrategyResult;
use crate::{
    solver::{Row, PosBitSet},
    Sudoku, Value, Pos,
};
use std::iter::FromIterator;

fn visit_remaining_patterns(mut row_iter: impl Iterator<Item = Row> + Clone,
    initial_candidates: &PosBitSet, remaining: PosBitSet, known: PosBitSet,
    all_accum: &mut PosBitSet, any_accum: &mut PosBitSet, cnt: &mut usize) -> bool
{
    while let Some(row) = row_iter.next() {
        let row_remaining = remaining & row.members_bitset();
        if row_remaining.is_empty() { continue }
        for pos in row_remaining.iter() {
            let mut known2 = known;
            let mut remaining2 = remaining;
            known2.insert(pos);
            remaining2.remove(pos);
            remaining2 &= !pos.neighbors_bitset();
            if !visit_remaining_patterns(row_iter.clone(), initial_candidates, remaining2, known2,
                all_accum, any_accum, cnt)
            {
                return false
            }
        }
        return true;
    }

    // Not a valid pattern
    if known.len() != Row::N { return true }

    *cnt += 1;
    *all_accum &= known;
    *any_accum |= known;

    // Keep going if we could make progress
    !all_accum.is_empty() || any_accum != initial_candidates
}

pub(crate) fn pattern_overlay_for_value(sudoku: &Sudoku, val: Value) -> Option<StrategyResult> {
    let candidates = sudoku.get_candidates_by_value(val);
    let knowns = PosBitSet::from_iter(Pos::iter().filter(|&pos| sudoku.get_value(pos) == Some(val)));

    let mut all_accum = candidates;
    let mut any_accum = PosBitSet::NONE;
    let mut cnt = 0;

    if visit_remaining_patterns(Row::iter(), &candidates, candidates, knowns,
        &mut all_accum, &mut any_accum, &mut cnt)
    {
        Some(StrategyResult::PatternOverlay {
            excluded_candidates: candidates.difference(any_accum).iter().map(|pos| (pos, val)).collect(),
            required_candidates: all_accum.iter().map(|pos| (pos, val)).collect(),
            value: val,
            remaining_patterns: cnt,
        })
    } else {
        None
    }
}

pub(crate) fn pattern_overlay(sudoku: &Sudoku) -> Option<StrategyResult> {
    for val in Value::iter() {
        if let ret@Some(_) = pattern_overlay_for_value(sudoku, val) { return ret }
    }
    None
}
