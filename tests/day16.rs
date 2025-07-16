use ahash::{AHashMap, AHashSet};
use itertools::{Itertools, iproduct};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[test]
fn day16() {
    // let txt = aoc::get_test_input().unwrap();
    let txt = aoc::get_input(16).unwrap();
    let m = txt.lines().map(|s| s.as_bytes()).collect::<Vec<_>>();

    let (h, w) = (m.len(), m[0].len());
    let (sx, sy) = iproduct!(0..h, 0..w)
        .filter(|&(x, y)| m[x][y] == b'S')
        .next()
        .unwrap();
    let (tx, ty) = iproduct!(0..h, 0..w)
        .filter(|&(x, y)| m[x][y] == b'E')
        .next()
        .unwrap();
    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    // part one
    let mut pq = BinaryHeap::new(); // (score, x, y, dir)
    let mut saw = AHashMap::new(); // (x, y, dir) => score
    pq.push((Reverse(0), sx, sy, 1));
    let mut best = None;
    while let Some((Reverse(score), x, y, dir)) = pq.pop() {
        if (x, y) == (tx, ty) {
            best = Some((dir, score)); // TODO: diff dir to txty
            break;
        }
        if !saw.contains_key(&(x, y, dir)) {
            saw.insert((x, y, dir), score);
        } else {
            continue;
        }
        for dd in [3, 0, 1] {
            let ndir = (dir + dd) % 4;
            let (nx, ny) = (x as isize + dirs[ndir].0, y as isize + dirs[ndir].1);
            let (nx, ny) = (nx as usize, ny as usize);
            if m[nx][ny] == b'#' {
                continue;
            }
            let nscore = score + if dd == 0 { 1 } else { 1001 };
            pq.push((Reverse(nscore), nx, ny, ndir));
        }
    }
    let (best_dir, best_score) = best.unwrap();
    dbg!(best_score);

    // part two
    let mut q = vec![];
    let mut found = AHashSet::new();
    q.push((tx, ty, best_dir, best_score));
    while let Some((x, y, dir, score)) = q.pop() {
        if !found.insert((x, y, dir)) {
            continue;
        }
        if (x, y) == (sx, sy) {
            continue;
        }
        let (px, py) = (x as isize - dirs[dir].0, y as isize - dirs[dir].1);
        let (px, py) = (px as usize, py as usize);
        for d in [3, 0, 1] {
            let pdir = (dir + d) % 4;
            if m[px][py] == b'#' {
                continue;
            }
            if let Some(&pscore) = saw.get(&(px, py, pdir)) {
                if pscore + if d == 0 { 1 } else { 1001 } == score {
                    q.push((px, py, pdir, pscore));
                }
            }
        }
    }
    dbg!(found.iter().map(|(x, y, _)| (x, y)).unique().count());
}
