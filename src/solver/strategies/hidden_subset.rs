use super::{StrategyResult, KnownSubsets};
use crate::{
    solver::{
        house::HouseIndexedSlice,
        House, PosBitSet, ValueBitSet,
    },
    Sudoku, Value,
};
use std::iter::FromIterator;

pub(crate) fn hidden_pair(sudoku: &Sudoku, known_subsets: &mut HouseIndexedSlice<KnownSubsets>) -> Option<StrategyResult> {
    hidden_subset(sudoku, known_subsets, 2)
}

pub(crate) fn hidden_triple(sudoku: &Sudoku, known_subsets: &mut HouseIndexedSlice<KnownSubsets>) -> Option<StrategyResult> {
    hidden_subset(sudoku, known_subsets, 3)
}

pub(crate) fn hidden_quadruple(sudoku: &Sudoku, known_subsets: &mut HouseIndexedSlice<KnownSubsets>) -> Option<StrategyResult> {
    hidden_subset(sudoku, known_subsets, 4)
}

fn hidden_subset(sudoku: &Sudoku, known_subsets: &mut HouseIndexedSlice<KnownSubsets>, subset_size: usize) -> Option<StrategyResult> {
    let mut ret = None;
    'outer:
    for house in House::iter() {
        let values: Vec<Value> = Value::iter()
            .filter(|&val| { let cnt = (sudoku.get_candidates_by_value(val) & house.members_bitset()).len(); cnt > 1 && cnt <= subset_size })
            .filter(|&val| !known_subsets[house].hidden.contains(val))
            .collect();
        if values.len() < subset_size { continue }

        for i1 in 0 .. values.len()+1-subset_size {
            let val1 = values[i1];
            let positions1 = sudoku.get_candidates_by_value(val1) & house.members_bitset();
            if positions1.len() > subset_size { continue }

            for i2 in i1+1 .. values.len()+2-subset_size {
                let val2 = values[i2];
                let positions2 = positions1 | (sudoku.get_candidates_by_value(val2) & house.members_bitset());
                if positions2.len() > subset_size { continue }

                if subset_size == 2 {
                    if let progress@Some(_) = handle_hidden_subset(sudoku, &mut known_subsets[house], &[val1, val2], positions2, house) {
                        ret = progress;
                        break 'outer;
                    }
                } else {
                    for i3 in i2+1 .. values.len()+3-subset_size {
                        let val3 = values[i3];
                        let positions3 = positions2 | (sudoku.get_candidates_by_value(val3) & house.members_bitset());
                        if positions3.len() > subset_size { continue }

                        if subset_size == 3 {
                            if let progress@Some(_) = handle_hidden_subset(sudoku, &mut known_subsets[house], &[val1, val2, val3], positions3, house) {
                                ret = progress;
                                break 'outer;
                            }
                        } else {
                            for i4 in i3+1 .. values.len()+4-subset_size {
                                let val4 = values[i4];
                                let positions4 = positions3 | (sudoku.get_candidates_by_value(val4) & house.members_bitset());
                                if positions4.len() > subset_size { continue }

                                if let progress@Some(_) = handle_hidden_subset(sudoku, &mut known_subsets[house], &[val1, val2, val3, val4], positions4, house) {
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
    ret
}

fn handle_hidden_subset(sudoku: &Sudoku, known_subsets: &mut KnownSubsets,
    values: &[Value], positions: PosBitSet, house: House) -> Option<StrategyResult>
{
    let mut excluded_candidates = Vec::new();
    let values_bitset = ValueBitSet::from_iter(values.iter().cloned());
    for pos in positions.iter() {
        for val in sudoku.get_candidates_by_pos(pos).difference(values_bitset).iter() {
            excluded_candidates.push((pos, val));
        }
    }

    for &val in values {
        known_subsets.hidden.insert(val);
    }
    known_subsets.naked |= positions;

    if !excluded_candidates.is_empty() {
        Some(StrategyResult::HiddenSubset {
            excluded_candidates,
            positions: positions.iter().collect(),
            values: values.iter().cloned().collect(),
            house
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Pos;

    fn check_hidden_subset_example(f: fn(&Sudoku, &mut HouseIndexedSlice<KnownSubsets>) -> Option<StrategyResult>,
        sudoku_line: &str, expected_res: Option<StrategyResult>)
    {
        let sudoku = Sudoku::from_line(sudoku_line).unwrap();
        let mut known_subsets = HouseIndexedSlice::from_slice([Default::default(); House::N]);
        let res = f(&sudoku, &mut known_subsets);
        println!("{:?}", sudoku);
        assert_eq!(res, expected_res);
    }

    #[test]
    fn test_hidden_pair_example1() {
        check_hidden_subset_example(hidden_pair,
            "......3.18..1.35.2.1...7864....2..1..75...286.218...3.14...69.3.3.4..1.82.....647",
            Some(StrategyResult::HiddenSubset {
                excluded_candidates: vec![
                    (Pos::new(40), Value::new(3)), (Pos::new(40), Value::new(8)),
                    (Pos::new(76), Value::new(4)), (Pos::new(76), Value::new(7)), (Pos::new(76), Value::new(8))
                ],
                positions: vec![Pos::new(40), Pos::new(76)],
                values: vec![Value::new(0), Value::new(2)],
                house: House::new(13)
            }));
    }

    #[test]
    fn test_hidden_pair_example2() {
        check_hidden_subset_example(hidden_pair,
            ".8..391646.1.42893349186...7..35.....13674....6592....1..46.......89..15.3.21.4..",
            Some(StrategyResult::HiddenSubset {
                excluded_candidates: vec![
                    (Pos::new(55), Value::new(1)), (Pos::new(55), Value::new(6)),
                    (Pos::new(72), Value::new(7))
                ],
                positions: vec![Pos::new(55), Pos::new(72)],
                values: vec![Value::new(4), Value::new(8)],
                house: House::new(24)
            }));
    }

    #[test]
    fn test_known_hidden_pair() {
        // Check that we don't attempt to scan the same subsets again
        let sudoku = Sudoku::from_line(".8..391646.1.42893349186...7..35.....13674....6592....1..46.......89..15.3.21.4..").unwrap();
        let mut known_subsets = HouseIndexedSlice::from_slice([Default::default(); House::N]);
        assert!(matches!(hidden_pair(&sudoku, &mut known_subsets), Some(_)));

        assert_ne!(known_subsets, HouseIndexedSlice::from_slice([Default::default(); House::N]));
        assert_eq!(hidden_pair(&sudoku, &mut known_subsets), None);
    }

    #[test]
    fn test_hidden_triple_example1() {
        check_hidden_subset_example(hidden_triple,
            "6.21598...574821...8.7362....48259..2963174..875694321.632715..72.5486.35.89637.2",
            Some(StrategyResult::HiddenSubset {
                excluded_candidates: vec![(Pos::new(7), Value::new(3)), (Pos::new(16), Value::new(8))],
                positions: vec![Pos::new(7), Pos::new(16), Pos::new(34)],
                values: vec![Value::new(2), Value::new(5), Value::new(6)],
                house: House::new(16)
            }));
    }

    #[test]
    fn test_hidden_quadruple_example1() {
        check_hidden_subset_example(hidden_quadruple,
            ".2....137.3....948.8....652261...489459.8.376873964215796...523548329761312657894",
            Some(StrategyResult::HiddenSubset {
                excluded_candidates: vec![
                    (Pos::new(3), Value::new(3)),
                    (Pos::new(12), Value::new(0)), (Pos::new(12), Value::new(6)),
                    (Pos::new(14), Value::new(0))
                ],
                positions: vec![Pos::new(3), Pos::new(5), Pos::new(12), Pos::new(14)],
                values: vec![Value::new(1), Value::new(4), Value::new(5), Value::new(7)],
                house: House::new(19)
            }));
    }

}
