use ahash::AHashMap;
use itertools::{Itertools, iproduct};
use nom::{
    IResult, Parser,
    character::complete::{i64, line_ending},
    multi::separated_list1 as sep1,
};
use std::iter::successors;

fn nxt(x: i64) -> i64 {
    let x = ((x << 6) ^ x) % 16777216;
    let x = ((x >> 5) ^ x) % 16777216;
    ((x << 11) ^ x) % 16777216
}

fn parse(s: &str) -> IResult<&str, Vec<i64>> {
    sep1(line_ending, i64).parse(s)
}

#[test]
fn day22() {
    // let txt = aoc::get_test_input().unwrap();
    let txt = aoc::get_input(22).unwrap();
    let (_, seeds) = parse(&txt).unwrap();
    let seqs = seeds
        .into_iter()
        .map(|x| {
            successors(Some(x), |&x| Some(nxt(x)))
                .take(2001)
                .collect_vec()
        })
        .collect_vec();
    let ans1 = seqs.iter().map(|s| s.last().unwrap()).sum::<i64>();
    dbg!(ans1);

    let pats = seqs
        .into_iter()
        .map(|seq| {
            let mut pat_to_first = AHashMap::new();
            for t in seq.windows(5) {
                let pat = (
                    t[1] % 10 - t[0] % 10,
                    t[2] % 10 - t[1] % 10,
                    t[3] % 10 - t[2] % 10,
                    t[4] % 10 - t[3] % 10,
                );
                if !pat_to_first.contains_key(&pat) {
                    pat_to_first.insert(pat, t[4] % 10);
                }
            }
            pat_to_first
        })
        .collect_vec();

    let mut ans2 = 0;
    for pat in iproduct!(-9..=9, -9..=9, -9..=9, -9..=9) {
        let mut tot = 0;
        for pat_to_first in &pats {
            if let Some(first) = pat_to_first.get(&pat) {
                tot += first;
            }
        }
        ans2 = ans2.max(tot);
    }
    dbg!(ans2);
}
