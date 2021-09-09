use super::StrategyResult;
use crate::{
    Sudoku, Pos,
};

pub(crate) fn xy_wing(sudoku: &Sudoku) -> Option<StrategyResult> {
    for xypos in Pos::iter() {
        let xyvals = sudoku.get_candidates_by_pos(xypos);
        if xyvals.len() != 2 { continue }

        let mut outer_iter = xypos.neighbors_iter();
        while let Some(xzpos) = outer_iter.next() {
            let xzvals = sudoku.get_candidates_by_pos(xzpos);
            if xzvals.len() != 2 { continue }
            if (xyvals & xzvals).len() != 1 { continue }
            let yzvals = xyvals.symmetric_difference(xzvals);

            for yzpos in outer_iter.clone() {
                if sudoku.get_candidates_by_pos(yzpos) != yzvals { continue }
                let z = xzvals.difference(xyvals).iter().next().unwrap();

                let mut excluded_candidates = Vec::new();
                for pos in (xzpos.neighbors_bitset() & yzpos.neighbors_bitset()).iter() {
                    if sudoku.get_candidates_by_pos(pos).contains(z) {
                        excluded_candidates.push((pos, z));
                    }
                }

                if !excluded_candidates.is_empty() {
                    let x = (xyvals & xzvals).iter().next().unwrap();
                    let y = xyvals.difference(xzvals).iter().next().unwrap();
                    return Some(StrategyResult::XyWing {
                        excluded_candidates,
                        positions: [xypos, xzpos, yzpos],
                        values: [x, y, z]
                    });
                }
            }
        }
    }
    None
}

pub(crate) fn xyz_wing(sudoku: &Sudoku) -> Option<StrategyResult> {
    for xyzpos in Pos::iter() {
        let xyzvals = sudoku.get_candidates_by_pos(xyzpos);
        if xyzvals.len() != 3 { continue }

        let mut outer_iter = xyzpos.neighbors_iter();
        while let Some(xzpos) = outer_iter.next() {
            let xzvals = sudoku.get_candidates_by_pos(xzpos);
            if xzvals.len() != 2 { continue }
            if (xyzvals & xzvals) != xzvals { continue }

            let y = xyzvals.difference(xzvals).iter().next().unwrap();

            for yzpos in outer_iter.clone() {
                let yzvals = sudoku.get_candidates_by_pos(yzpos);
                if yzvals.len() != 2 { continue }
                if !yzvals.contains(y) { continue }
                if (yzvals & xzvals).is_empty() { continue }

                let z = (yzvals & xzvals).iter().next().unwrap();

                let mut excluded_candidates = Vec::new();
                for pos in (xyzpos.neighbors_bitset() & xzpos.neighbors_bitset() & yzpos.neighbors_bitset()).iter() {
                    if sudoku.get_candidates_by_pos(pos).contains(z) {
                        excluded_candidates.push((pos, z));
                    }
                }

                if !excluded_candidates.is_empty() {
                    let x = xzvals.difference(yzvals).iter().next().unwrap();
                    return Some(StrategyResult::XyzWing {
                        excluded_candidates,
                        positions: [xyzpos, xzpos, yzpos],
                        values: [x, y, z]
                    });
                }
            }
        }
    }
    None
}

pub(crate) fn wxyz_wing(sudoku: &Sudoku) -> Option<StrategyResult> {
    for wxyzpos in Pos::iter() {
        let wxyzvals = sudoku.get_candidates_by_pos(wxyzpos);
        if wxyzvals.len() != 3 && wxyzvals.len() != 4 { continue }

        let mut iter1 = wxyzpos.neighbors_iter();
        while let Some(wzpos) = iter1.next() {
            let wzvals = sudoku.get_candidates_by_pos(wzpos);
            if wzvals.len() != 2 { continue }
            if wxyzvals.difference(wzvals).len() != 2 { continue }

            let mut iter2 = iter1.clone();
            while let Some(xzpos) = iter2.next() {
                let xzvals = sudoku.get_candidates_by_pos(xzpos);
                if xzvals.len() != 2 { continue }
                if (wzvals & xzvals).is_empty() || wzvals == xzvals { continue }

                let z = (wzvals & xzvals).iter().next().unwrap();
                let x = xzvals.difference(wzvals).iter().next().unwrap();
                let w = wzvals.difference(xzvals).iter().next().unwrap();

                if !wxyzvals.contains(x) { continue }
                let y = wxyzvals.difference(wzvals | xzvals).iter().next().unwrap();

                for yzpos in iter2.clone() {
                    let yzvals = sudoku.get_candidates_by_pos(yzpos);
                    if yzvals.len() != 2 { continue }
                    if !yzvals.contains(y) || !yzvals.contains(z) { continue }

                    let mut excluded_candidates = Vec::new();
                    let mut neighbors = wzpos.neighbors_bitset() & xzpos.neighbors_bitset() & yzpos.neighbors_bitset();
                    if wxyzvals.contains(z) {
                        neighbors &= wxyzpos.neighbors_bitset();
                    }
                    for pos in neighbors.iter() {
                        if sudoku.get_candidates_by_pos(pos).contains(z) {
                            excluded_candidates.push((pos, z));
                        }
                    }

                    if !excluded_candidates.is_empty() {
                        return Some(StrategyResult::WxyzWing {
                            excluded_candidates,
                            positions: [wxyzpos, wzpos, xzpos, yzpos],
                            values: [w, x, y, z]
                        });
                    }
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        solver::tests::check_example,
        Value,
    };

    #[test]
    fn test_xy_wing_example1() {
        check_example(xy_wing,
            ".7.39164...16.459769475.13.4..219876926875413817463259..91..76474.5.6921162947385",
            Some(StrategyResult::XyWing {
                excluded_candidates: vec![(Pos::new(28), Value::new(2))],
                positions: [Pos::new(2), Pos::new(10), Pos::new(29)],
                values: [Value::new(7), Value::new(4), Value::new(2)]
            }));
    }

    #[test]
    fn test_xyz_wing_example1() {
        check_example(xyz_wing,
            ".258..34..8...4..14.1..37...7.5..2.45.8742.131423.9.7..1.4958322591384678346271..",
            Some(StrategyResult::XyzWing {
                excluded_candidates: vec![(Pos::new(9), Value::new(5))],
                positions: [Pos::new(0), Pos::new(19), Pos::new(54)],
                values: [Value::new(8), Value::new(6), Value::new(5)]
            }));
    }

    #[test]
    fn test_wxyz_wing_example1() {
        // pivot includes only wxy
        check_example(wxyz_wing,
            ".3.61.....1.3.9...9..7...13279456138..1238957583971426126843...358197264794562381",
            Some(StrategyResult::WxyzWing {
                excluded_candidates: vec![(Pos::new(7), Value::new(3))],
                positions: [Pos::new(6), Pos::new(0), Pos::new(5), Pos::new(16)],
                values: [Value::new(7), Value::new(4), Value::new(6), Value::new(3)]
            }));
    }

    #[test]
    fn test_wxyz_wing_example2() {
        // pivot includes wxyz
        check_example(wxyz_wing,
            "..196.74.6..7.481..2.5813963....62.1.1....6.8.6...5439..8647123246..95871..258964",
            Some(StrategyResult::WxyzWing {
                excluded_candidates: vec![(Pos::new(29), Value::new(6))],
                positions: [Pos::new(28), Pos::new(31), Pos::new(34), Pos::new(45)],
                values: [Value::new(8), Value::new(4), Value::new(7), Value::new(6)]
            }));
    }
}
