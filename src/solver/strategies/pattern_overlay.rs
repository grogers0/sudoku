use std::collections::HashMap;
use crate::solver::{
    Row, PosBitSet,
};

fn generate_all_patterns() -> Vec<PosBitSet> {
    fn recur_internal(givens: PosBitSet, covered: PosBitSet, accum: &mut Vec<PosBitSet>) {
        if covered == PosBitSet::ALL {
            accum.push(givens);
            return;
        }
        let row = Row::iter()
            .filter(|row| (givens & row.members_bitset()).is_empty())
            .next().unwrap();
        for pos in (!covered & row.members_bitset()).iter() {
            let mut givens2 = givens;
            let mut covered2 = covered;
            givens2.insert(pos);
            covered2.insert(pos);
            covered2 |= pos.neighbors_bitset();
            recur_internal(givens2, covered2, accum);
        }
    }
    const NUM_PATTERNS: usize = 46656;
    let mut accum = Vec::with_capacity(NUM_PATTERNS);
    recur_internal(PosBitSet::NONE, PosBitSet::NONE, &mut accum);
    assert_eq!(NUM_PATTERNS, accum.len());
    accum
}

/// For a given pattern, generate all combinations of givens
fn generate_givens_combinations(pattern: PosBitSet) -> Vec<PosBitSet> {
    fn recur_internal(givens: PosBitSet, mut pattern: PosBitSet, accum: &mut Vec<PosBitSet>) {
        accum.push(givens);
        for pos in pattern.iter() {
            let mut givens2 = givens;
            givens2.insert(pos);
            pattern.remove(pos);
            recur_internal(givens2, pattern, accum);
        }
    }
    const NUM_COMBINATIONS: usize = 512; // sum from i in 0..=9 of (9 choose i)
    let mut accum = Vec::with_capacity(NUM_COMBINATIONS);
    recur_internal(PosBitSet::NONE, pattern, &mut accum);
    assert_eq!(NUM_COMBINATIONS, accum.len());
    accum
}


/// Map from the set of givens to the set of all legal patterns
#[static_init::dynamic]
static PATTERNS: HashMap<PosBitSet, Vec<PosBitSet>> = {
    let start = std::time::Instant::now();
    let patterns = generate_all_patterns();
    let mut givens_patterns = HashMap::with_capacity(patterns.len());
    let mut cnt = 0usize;
    for pattern in patterns {
        for givens in generate_givens_combinations(pattern) {
            let remaining = pattern.difference(givens);
            cnt += 1;
            givens_patterns.entry(givens).or_insert(Vec::new()).push(remaining);
        }
    }
    eprintln!("computed {} patterns in {:?}", cnt, start.elapsed());
    givens_patterns
};

