use ahash::{AHashMap, AHashSet};
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{i64, line_ending},
    multi::separated_list1 as sep1,
    sequence::separated_pair,
};
use std::collections::VecDeque;

fn parse(s: &str) -> IResult<&str, Vec<(i64, i64)>> {
    sep1(line_ending, separated_pair(i64, tag(","), i64)).parse(s)
}

#[test]
fn day18() {
    // let txt = aoc::get_test_input().unwrap();
    // let n = 7;
    // let limit = 12;
    let txt = aoc::get_input(18).unwrap();
    let n = 71;
    let limit = 1024;

    let (_, poses) = parse(&txt).unwrap();
    let blocks = poses
        .iter()
        .cloned()
        .enumerate()
        .map(|(idx, (y, x))| ((x, y), idx))
        .collect::<AHashMap<_, _>>();

    let bfs = |limit: usize| -> Option<usize> {
        let mut q = VecDeque::new();
        let mut saw = AHashSet::new();
        q.push_back((0, 0, 0));
        while let Some((x, y, steps)) = q.pop_front() {
            if (x, y) == (n - 1, n - 1) {
                return Some(steps);
            }
            if !saw.insert((x, y)) {
                continue;
            }
            for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let (nx, ny) = (x + dx, y + dy);
                if nx < 0 || ny < 0 || nx == n || ny == n {
                    continue;
                }
                if let Some(&idx) = blocks.get(&(nx, ny)) {
                    if idx < limit {
                        continue;
                    }
                }
                q.push_back((nx, ny, steps + 1));
            }
        }
        None
    };

    dbg!(bfs(limit));

    let mut l = 0;
    let mut r = blocks.len();
    while l + 1 != r {
        let m = (l + r) / 2;
        if bfs(m).is_some() {
            l = m;
        } else {
            r = m;
        }
    }
    dbg!(poses[l]);
}
