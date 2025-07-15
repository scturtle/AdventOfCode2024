use itertools::{Itertools, iproduct};

#[test]
fn day15() {
    // let txt = aoc::get_test_input().unwrap();
    let txt = aoc::get_input(15).unwrap();

    let m = txt
        .lines()
        .take_while(|l| l.starts_with('#'))
        .map(|l| l.bytes().collect_vec())
        .collect_vec();
    let (h, w) = (m.len(), m[0].len());

    let moves = txt
        .lines()
        .skip_while(|l| l.starts_with('#') || l.is_empty())
        .flat_map(|l| l.chars())
        .collect_vec();

    let (sx, sy) = iproduct!(0..h, 0..w)
        .filter(|&(x, y)| m[x][y] == b'@')
        .next()
        .unwrap();

    let dir = [('^', (-1, 0)), ('v', (1, 0)), ('<', (0, -1)), ('>', (0, 1))]
        .into_iter()
        .collect::<ahash::AHashMap<_, _>>();

    // part one
    {
        let mut m = m.clone();
        let (mut sx, mut sy) = (sx as i32, sy as i32);
        for &mov in &moves {
            let (dx, dy) = dir[&mov];
            let (mut ex, mut ey) = (sx + dx, sy + dy);
            while m[ex as usize][ey as usize] == b'O' {
                (ex, ey) = (ex + dx, ey + dy);
            }
            if m[ex as usize][ey as usize] == b'#' {
                continue;
            }
            while (ex, ey) != (sx, sy) {
                m[ex as usize][ey as usize] = m[(ex - dx) as usize][(ey - dy) as usize];
                (ex, ey) = (ex - dx, ey - dy);
            }
            m[sx as usize][sy as usize] = b'.';
            (sx, sy) = (sx + dx, sy + dy);
        }
        let ans1 = iproduct!(0..h, 0..w)
            .filter(|&(x, y)| m[x][y] == b'O')
            .map(|(x, y)| x * 100 + y)
            .sum::<usize>();
        dbg!(ans1);
    }

    // part two
    {
        let mut m = m
            .iter()
            .map(|l| {
                let mut l2 = vec![];
                for b in l {
                    match b {
                        b'#' => {
                            l2.push(b'#');
                            l2.push(b'#');
                        }
                        b'.' => {
                            l2.push(b'.');
                            l2.push(b'.');
                        }
                        b'@' => {
                            l2.push(b'@');
                            l2.push(b'.');
                        }
                        b'O' => {
                            l2.push(b'[');
                            l2.push(b']');
                        }
                        _ => unreachable!(),
                    }
                }
                l2
            })
            .collect_vec();

        let (mut sx, mut sy) = (sx as i32, 2 * sy as i32);
        for &mov in &moves {
            let (dx, dy) = dir[&mov];
            if mov == '<' || mov == '>' {
                let (mut ex, mut ey) = (sx + dx, sy + dy);
                while m[ex as usize][ey as usize] == b'[' || m[ex as usize][ey as usize] == b']' {
                    (ex, ey) = (ex + dx, ey + dy);
                }
                if m[ex as usize][ey as usize] == b'#' {
                    continue;
                }
                while (ex, ey) != (sx, sy) {
                    m[ex as usize][ey as usize] = m[(ex - dx) as usize][(ey - dy) as usize];
                    (ex, ey) = (ex - dx, ey - dy);
                }
                m[sx as usize][sy as usize] = b'.';
                (sx, sy) = (sx + dx, sy + dy);
            } else {
                assert!(mov == '^' || mov == 'v');
                let (ex, ey) = (sx + dx, sy + dy);
                let mut q = std::collections::VecDeque::new();
                let mut ok = vec![];
                q.push_back(((sx, sy), (ex, ey)));
                while let Some(((fx, fy), (tx, ty))) = q.pop_front() {
                    if ok.contains(&((fx, fy), (tx, ty))) {
                        continue;
                    }
                    if m[tx as usize][ty as usize] == b'#' {
                        q.push_back(((fx, fy), (tx, ty)));
                        break;
                    }
                    if m[tx as usize][ty as usize] == b'.' {
                        ok.push(((fx, fy), (tx, ty)));
                        continue;
                    }
                    if m[tx as usize][ty as usize] == b'[' {
                        ok.push(((fx, fy), (tx, ty)));
                        q.push_back(((tx, ty), (tx + dx, ty)));
                        q.push_back(((tx, ty + 1), (tx + dx, ty + 1)));
                    }
                    if m[tx as usize][ty as usize] == b']' {
                        ok.push(((fx, fy), (tx, ty)));
                        q.push_back(((tx, ty), (tx + dx, ty)));
                        q.push_back(((tx, ty - 1), (tx + dx, ty - 1)));
                    }
                }
                if q.is_empty() {
                    for ((fx, fy), (tx, ty)) in ok.into_iter().rev() {
                        m[tx as usize][ty as usize] = m[fx as usize][fy as usize];
                        m[fx as usize][fy as usize] = b'.';
                    }
                    (sx, sy) = (sx + dx, sy + dy);
                }
            }
            // println!("{}\n", m.iter().map(|l| String::from_utf8_lossy(l)).collect_vec().join("\n"));
        }
        let ans2 = iproduct!(0..h, 0..2 * w)
            .filter(|&(x, y)| m[x][y] == b'[')
            .map(|(x, y)| x * 100 + y)
            .sum::<usize>();
        dbg!(ans2);
    }
}
