use ahash::AHashSet;
use itertools::{Itertools, iproduct};

fn around(i: usize, j: usize, h: usize, w: usize) -> impl Iterator<Item = (usize, usize)> {
    [(0, 1), (0, -1), (-1, 0), (1, 0)]
        .into_iter()
        .filter_map(move |(ai, aj)| {
            let (ni, nj) = (i as isize + ai, j as isize + aj);
            if ni >= 0 && nj >= 0 && ni < h as isize && nj < w as isize {
                Some((ni as usize, nj as usize))
            } else {
                None
            }
        })
}

#[test]
fn day12() {
    // let txt = aoc::get_test_input().unwrap();
    let txt = aoc::get_input(12).unwrap();
    let m = txt.lines().map(|s| s.as_bytes()).collect::<Vec<_>>();
    let (h, w) = (m.len(), m[0].len());

    let mut ans1 = 0;
    let mut ans2 = 0;
    let mut saw = AHashSet::<(usize, usize)>::new();
    for (x, y) in iproduct!(0..h, 0..w) {
        if saw.contains(&(x, y)) {
            continue;
        }

        // bfs to collect all blocks in a region
        let c = m[x][y];
        let mut q = vec![];
        let mut region = AHashSet::<(usize, usize)>::new();
        q.push((x, y));
        region.insert((x, y));
        while let Some((x, y)) = q.pop() {
            for (nx, ny) in around(x, y, h, w) {
                if m[nx][ny] == c && region.insert((nx, ny)) {
                    q.push((nx, ny));
                }
            }
        }
        saw.extend(&region);

        // collect all edges in a region
        let mut edges = vec![];
        for (i, j) in region.clone() {
            for (ai, aj, pi0, pj0, pi1, pj1, dir) in [
                (-1, 0, 0, 0, 0, 1, 'U'),
                (0, -1, 0, 0, 1, 0, 'L'),
                (1, 0, 1, 0, 1, 1, 'D'),
                (0, 1, 0, 1, 1, 1, 'R'),
            ] {
                let (ni, nj) = (i as isize + ai, j as isize + aj);
                if !(ni >= 0
                    && nj >= 0
                    && ni < h as isize
                    && nj < w as isize
                    && region.contains(&(ni as usize, nj as usize)))
                {
                    // there is no same block on this side and there must be an edge
                    edges.push((i + pi0, j + pj0, i + pi1, j + pj1, dir));
                }
            }
        }
        // part one
        ans1 += region.len() * edges.len();

        // part two
        let mut n_sides = 0;
        for dir in ['L', 'R'] {
            n_sides += edges
                .iter()
                .filter(|t| t.4 == dir)
                .sorted_by_key(|t| (t.1, t.0))
                .chunk_by(|t| t.1)
                .into_iter()
                .map(|(_, es)| {
                    es.tuple_windows()
                        .filter(|(e1, e2)| e2.0 > e1.0 + 1)
                        .count()
                        + 1
                })
                .sum::<usize>();
        }
        for dir in ['U', 'D'] {
            n_sides += edges
                .iter()
                .filter(|t| t.4 == dir)
                .sorted_by_key(|t| (t.0, t.1))
                .chunk_by(|t| t.0)
                .into_iter()
                .map(|(_, es)| {
                    es.tuple_windows()
                        .filter(|(e1, e2)| e2.1 > e1.1 + 1)
                        .count()
                        + 1
                })
                .sum::<usize>();
        }
        ans2 += region.len() * n_sides;
    }
    dbg!(ans1);
    dbg!(ans2);
}
