use super::StrategyResult;
use crate::{
    solver::{House, PosBitSet, ValueBitSet},
    Sudoku, Pos,
};

pub(crate) fn naked_pair(sudoku: &Sudoku, naked_subset_positions: &mut PosBitSet) -> Option<StrategyResult> {
    naked_subset(sudoku, naked_subset_positions, 2)
}

pub(crate) fn naked_triple(sudoku: &Sudoku, naked_subset_positions: &mut PosBitSet) -> Option<StrategyResult> {
    naked_subset(sudoku, naked_subset_positions, 3)
}

pub(crate) fn naked_quad(sudoku: &Sudoku, naked_subset_positions: &mut PosBitSet) -> Option<StrategyResult> {
    naked_subset(sudoku, naked_subset_positions, 4)
}

// NOTE - `naked_subset_positions` saves previously found subsets so we don't bother scanning
// through them for different subsets. E.g. a naked pair can't be part of a naked triple, so ignore
// it.
fn naked_subset(sudoku: &Sudoku, naked_subset_positions_orig: &mut PosBitSet, subset_size: usize) -> Option<StrategyResult> {
    let mut naked_subset_positions = PosBitSet::NONE;
    let mut ret = None;
    'outer:
    for house in House::iter() {
        let positions: Vec<Pos> = house.members_iter()
            .filter(|&pos| { let cnt = sudoku.get_candidates_by_pos(pos).len(); cnt > 1 && cnt <= subset_size })
            .filter(|&pos| !naked_subset_positions_orig.contains(pos))
            .collect();
        if positions.len() < subset_size { continue }

        for i1 in 0 .. positions.len() - 1 {
            let pos1 = positions[i1];
            let values1 = sudoku.get_candidates_by_pos(pos1);
            if values1.len() > subset_size { continue }

            for i2 in i1+1 .. positions.len() {
                let pos2 = positions[i2];
                let values2 = values1 | sudoku.get_candidates_by_pos(pos2);
                if values2.len() > subset_size { continue }

                if subset_size == 2 {
                    if let progress@Some(_) = handle_naked_subset(sudoku, &mut naked_subset_positions, &[pos1, pos2], values2) {
                        ret = progress;
                        break 'outer;
                    }
                } else {
                    for i3 in i2+1 .. positions.len() {
                        let pos3 = positions[i3];
                        let values3 = values2 | sudoku.get_candidates_by_pos(pos3);
                        if values3.len() > subset_size { continue }

                        if subset_size == 3 {
                            if let progress@Some(_) = handle_naked_subset(sudoku, &mut naked_subset_positions, &[pos1, pos2, pos3], values3) {
                                ret = progress;
                                break 'outer;
                            }
                        } else {
                            for i4 in i3+1 .. positions.len() {
                                let pos4 = positions[i4];
                                let values4 = values3 | sudoku.get_candidates_by_pos(pos4);
                                if values4.len() > subset_size { continue }

                                if let progress@Some(_) = handle_naked_subset(sudoku, &mut naked_subset_positions, &[pos1, pos2, pos3, pos4], values4) {
                                    ret = progress;
                                    break 'outer;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    *naked_subset_positions_orig |= naked_subset_positions;
    ret
}

fn handle_naked_subset(sudoku: &Sudoku, naked_subset_positions: &mut PosBitSet, positions: &[Pos], values: ValueBitSet) -> Option<StrategyResult> {
    // We recalculate the neighbors instead of using the house because we might have a pair/triple
    // in the intersection of a block/line: just the house wouldn't give all neighbors, and we'd
    // have to check the other house the next solve iteration.
    let neighbors = positions.iter().fold(PosBitSet::ALL, |neighbors, pos| neighbors & pos.neighbors_bitset());
    let mut exclusions = Vec::new();
    for neigh_pos in neighbors.iter() {
        for val in (values & sudoku.get_candidates_by_pos(neigh_pos)).iter() {
            exclusions.push((neigh_pos, val));
        }
    }
    for &pos in positions {
        naked_subset_positions.insert(pos);
    }
    if !exclusions.is_empty() {
        Some(StrategyResult::NakedSubset {
            exclusions,
            positions: positions.iter().cloned().collect(),
            values: values.iter().collect()
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Value;

    fn check_naked_subset_example(f: fn(&Sudoku, &mut PosBitSet) -> Option<StrategyResult>, sudoku_line: &str, expected_res: Option<StrategyResult>) {
        let sudoku = Sudoku::from_line(sudoku_line).unwrap();
        let mut naked_subset_positions = PosBitSet::NONE;
        let res = f(&sudoku, &mut naked_subset_positions);
        println!("{:?}", sudoku);
        assert_eq!(res, expected_res);
    }

    #[test]
    fn test_naked_pair_example1() {
        check_naked_subset_example(naked_pair,
            "634859721172346859598172..441..25.8.35.98..1282..315.7.83.6.1.57415982...65.13..8",
            Some(StrategyResult::NakedSubset {
                exclusions: vec![(Pos::new(52), Value::new(5))],
                positions: vec![Pos::new(25), Pos::new(70)],
                values: vec![Value::new(2), Value::new(5)]
            }));
    }

    #[test]
    fn test_naked_pair_example2() {
        check_naked_subset_example(naked_pair,
            ".7...12.4.5.9..1...1......353681.4..749..58..821.....5.951.87...6...25...87...34.",
            Some(StrategyResult::NakedSubset {
                exclusions: vec![
                    (Pos::new(54), Value::new(1)), (Pos::new(58), Value::new(5)),
                    (Pos::new(80), Value::new(1)), (Pos::new(80), Value::new(5)),
                ],
                positions: vec![Pos::new(61), Pos::new(62)],
                values: vec![Value::new(1), Value::new(5)]
            }));

    }

    #[test]
    fn test_naked_subset_positions() {
        // Check that we don't attempt to scan the same subsets again
        let sudoku = Sudoku::from_line(".7...12.4.5.9..1...1......353681.4..749..58..821.....5.951.87...6...25...87...34.").unwrap();
        let mut naked_subset_positions = PosBitSet::NONE;
        assert!(matches!(naked_pair(&sudoku, &mut naked_subset_positions), Some(_)));

        assert_ne!(naked_subset_positions, PosBitSet::NONE);
        assert_eq!(naked_pair(&sudoku, &mut naked_subset_positions), None);
    }

    #[test]
    fn test_naked_triple_example1() {
        check_naked_subset_example(naked_triple,
            "7..6....3..3..1..5..87.3.2.5..12....31...76...9.3...17..5.....4.4.93.....3.......",
            Some(StrategyResult::NakedSubset {
                exclusions: vec![
                    (Pos::new(2), Value::new(1)), (Pos::new(9), Value::new(1)), (Pos::new(9), Value::new(5)),
                    (Pos::new(18), Value::new(5)), (Pos::new(28), Value::new(5)),
                    (Pos::new(55), Value::new(1)), (Pos::new(55), Value::new(5))
                ],
                positions: vec![Pos::new(1), Pos::new(10), Pos::new(19)],
                values: vec![Value::new(1), Value::new(4), Value::new(5)]
            }));
    }

    #[test]
    fn test_naked_quad_example1() {
        check_naked_subset_example(naked_quad,
            "81..439565.9..63..36.9.5...9364512874..83761917869253468......22...681.57.....863",
            Some(StrategyResult::NakedSubset {
                exclusions: vec![
                    (Pos::new(56), Value::new(0)), (Pos::new(56), Value::new(3)),
                    (Pos::new(57), Value::new(0)), (Pos::new(57), Value::new(6))
                ],
                positions: vec![Pos::new(58), Pos::new(59), Pos::new(60), Pos::new(61)],
                values: vec![Value::new(0), Value::new(3), Value::new(6), Value::new(8)]
            }));
    }
}
