use itertools::iproduct;

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
fn day10() {
    // let txt = aoc::get_test_input().unwrap();
    let txt = aoc::get_input(10).unwrap();
    let m = txt.lines().map(|s| s.as_bytes()).collect::<Vec<_>>();
    let (h, w) = (m.len(), m[0].len());

    // part one
    let mut ans1 = 0;
    for (x, y) in iproduct!(0..h, 0..w) {
        if m[x][y] == b'0' {
            let mut q = vec![];
            let mut saw = ahash::AHashSet::new();
            q.push((x, y, b'0'));
            saw.insert((x, y));
            while let Some((x, y, cur)) = q.pop() {
                if cur == b'9' {
                    ans1 += 1;
                } else {
                    for (nx, ny) in around(x, y, h, w) {
                        if m[nx][ny] == cur + 1 && saw.insert((nx, ny)) {
                            q.push((nx, ny, m[nx][ny]));
                        }
                    }
                }
            }
        }
    }
    dbg!(ans1);

    // part two
    let mut ans2 = 0;
    for (x, y) in iproduct!(0..h, 0..w) {
        if m[x][y] == b'0' {
            let mut q = vec![];
            q.push((x, y, b'0'));
            while let Some((x, y, cur)) = q.pop() {
                if cur == b'9' {
                    ans2 += 1;
                } else {
                    for (nx, ny) in around(x, y, h, w) {
                        if m[nx][ny] == cur + 1 {
                            q.push((nx, ny, m[nx][ny]));
                        }
                    }
                }
            }
        }
    }
    dbg!(ans2);
}
