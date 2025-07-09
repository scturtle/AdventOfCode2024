use ahash::AHashSet;
use itertools::{Itertools, iproduct};

#[test]
fn day06() {
    // let txt = aoc::get_test_input().unwrap();
    let txt = aoc::get_input(6).unwrap();
    let m = txt.lines().map(|s| s.as_bytes()).collect::<Vec<_>>();

    let (init_x, init_y) = {
        let mut init = (0, 0);
        for (i, j) in iproduct!(0..m.len(), 0..m[0].len()) {
            if m[i][j] == b'^' {
                init = (i, j);
            }
        }
        init
    };

    let mut saw = AHashSet::new();
    let mut dir = 0;
    let turn = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let (mut x, mut y) = (init_x as isize, init_y as isize);
    loop {
        saw.insert((x, y));
        let (nx, ny) = (x + turn[dir].0, y + turn[dir].1);
        if nx < 0 || ny < 0 || nx >= m.len() as isize || ny >= m[0].len() as isize {
            break;
        }
        if m[nx as usize][ny as usize] == b'#' {
            dir = (dir + 1) % 4;
            continue;
        }
        (x, y) = (nx, ny);
    }
    dbg!(saw.len());
    let all_poses = saw.into_iter().collect_vec();

    let is_loop = |extra_block_x, extra_block_y| -> bool {
        let mut saw = AHashSet::new();
        let mut dir = 0;
        let (mut x, mut y) = (init_x as isize, init_y as isize);
        loop {
            if !saw.insert((x, y, dir)) {
                return true;
            }
            let (nx, ny) = (x + turn[dir].0, y + turn[dir].1);
            if nx < 0 || ny < 0 || nx >= m.len() as isize || ny >= m[0].len() as isize {
                return false;
            }
            if (nx == extra_block_x && ny == extra_block_y) || m[nx as usize][ny as usize] == b'#' {
                dir = (dir + 1) % 4;
                continue;
            }
            (x, y) = (nx, ny);
        }
    };
    dbg!(
        all_poses
            .into_iter()
            .filter(|&(x, y)| is_loop(x, y))
            .count()
    );
}
