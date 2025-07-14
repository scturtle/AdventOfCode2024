use itertools::iproduct;

#[test]
fn day08() {
    // let txt = aoc::get_test_input().unwrap();
    let txt = aoc::get_input(8).unwrap();
    let m = txt.lines().map(|s| s.as_bytes()).collect::<Vec<_>>();
    let n_rows = m.len();
    let n_cols = m[0].len();
    let is_legal = |x: isize, y: isize| -> bool {
        x >= 0 && y >= 0 && x < n_rows as isize && y < n_cols as isize
    };
    let mut antennas = ahash::AHashMap::<u8, Vec<(isize, isize)>>::new();
    for (x, y) in iproduct!(0..n_rows, 0..n_cols) {
        if m[x][y] != b'.' {
            antennas
                .entry(m[x][y])
                .or_default()
                .push((x as isize, y as isize));
        }
    }
    let mut ans1 = ahash::AHashSet::new();
    let mut ans2 = ahash::AHashSet::new();
    for v in antennas.values() {
        for i in 0..v.len() {
            ans2.insert(v[i]); // part two
            for j in i + 1..v.len() {
                let (i1, j1) = v[i];
                let (i2, j2) = v[j];
                let (di, dj) = (i2 - i1, j2 - j1);
                // part one
                if is_legal(i2 + di, j2 + dj) {
                    ans1.insert((i2 + di, j2 + dj));
                }
                if is_legal(i1 - di, j1 - dj) {
                    ans1.insert((i1 - di, j1 - dj));
                }
                // part two
                let (mut ti, mut tj) = (i2 + di, j2 + dj);
                while is_legal(ti, tj) {
                    ans2.insert((ti, tj));
                    (ti, tj) = (ti + di, tj + dj);
                }
                let (mut ti, mut tj) = (i1 - di, j1 - dj);
                while is_legal(ti, tj) {
                    ans2.insert((ti, tj));
                    (ti, tj) = (ti - di, tj - dj);
                }
            }
        }
    }
    dbg!(ans1.len());
    dbg!(ans2.len());
}
