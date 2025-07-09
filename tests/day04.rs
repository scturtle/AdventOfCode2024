use itertools::iproduct;

#[test]
fn day04() {
    // let txt = aoc::get_test_input().unwrap();
    let txt = aoc::get_input(4).unwrap();
    let m = txt.lines().map(|s| s.as_bytes()).collect::<Vec<_>>();
    let poses = [
        [(0, 0), (0, 1), (0, 2), (0, 3)],
        [(0, 0), (1, 0), (2, 0), (3, 0)],
        [(0, 0), (1, 1), (2, 2), (3, 3)],
        [(3, 0), (2, 1), (1, 2), (0, 3)],
    ];
    let mut ans1 = 0;
    for (i, j) in iproduct!(0..m.len(), 0..m[0].len()) {
        let i = i as isize;
        let j = j as isize;
        for p in &poses {
            let p = p
                .iter()
                .map(|&(ii, jj)| (i + ii, j + jj))
                .collect::<Vec<_>>();
            if p.iter()
                .any(|&(i, j)| i < 0 || j < 0 || i >= m.len() as isize || j >= m[0].len() as isize)
            {
                continue;
            }
            let s = p
                .iter()
                .map(|&(i, j)| m[i as usize][j as usize])
                .collect::<Vec<_>>();
            if s == [b'X', b'M', b'A', b'S'] || s == [b'S', b'A', b'M', b'X'] {
                ans1 += 1;
            }
        }
    }
    dbg!(ans1);

    let mut ans2 = 0;
    let poses = [
        [(0, 0), (2, 0), (1, 1), (0, 2), (2, 2)],
        [(0, 0), (0, 2), (1, 1), (2, 0), (2, 2)],
        [(2, 0), (2, 2), (1, 1), (0, 0), (0, 2)],
        [(0, 2), (2, 2), (1, 1), (0, 0), (2, 0)],
    ];
    for (i, j) in iproduct!(0..m.len(), 0..m[0].len()) {
        let i = i as isize;
        let j = j as isize;
        for p in &poses {
            let p = p
                .iter()
                .map(|&(ii, jj)| (i + ii, j + jj))
                .collect::<Vec<_>>();
            if p.iter()
                .any(|&(i, j)| i < 0 || j < 0 || i >= m.len() as isize || j >= m[0].len() as isize)
            {
                continue;
            }
            let s = p
                .iter()
                .map(|&(i, j)| m[i as usize][j as usize])
                .collect::<Vec<_>>();
            if s == [b'M', b'M', b'A', b'S', b'S'] {
                ans2 += 1;
            }
        }
    }
    dbg!(ans2);
}
