use ahash::AHashMap;
use itertools::iproduct;
use std::collections::VecDeque;

#[test]
fn day20() {
    // let txt = aoc::get_test_input().unwrap();
    let txt = aoc::get_input(20).unwrap();
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

    let dis_to = |ax, ay| {
        let mut dis_to_s = AHashMap::<(usize, usize), usize>::new();
        let mut q = VecDeque::new();
        q.push_back((ax, ay, 0));
        while let Some((x, y, steps)) = q.pop_front() {
            if dis_to_s.contains_key(&(x, y)) {
                continue;
            }
            dis_to_s.insert((x, y), steps);
            for (dx, dy) in dirs {
                let (nx, ny) = (x as isize + dx, y as isize + dy);
                if nx < 0 || ny < 0 || nx == h as isize || ny == w as isize {
                    continue;
                }
                let (nx, ny) = (nx as usize, ny as usize);
                if m[nx][ny] == b'#' {
                    continue;
                }
                q.push_back((nx, ny, steps + 1));
            }
        }
        dis_to_s
    };

    let dis_to_s = dis_to(sx, sy);
    let dis_to_t = dis_to(tx, ty);
    let baseline = dis_to_s[&(tx, ty)];

    // NOTE: cheat start and end must be space (./S/E)
    let mut ans = 0;
    for (wx, wy) in iproduct!(0..h, 0..w).filter(|&(ex, ey)| m[ex][ey] == b'.' || m[ex][ey] == b'S')
    {
        if let Some(s_to_wall) = dis_to_s.get(&(wx, wy)) {
            for (ex, ey) in iproduct!(
                wx as isize - 20..=wx as isize + 20,
                wy as isize - 20..=wy as isize + 20,
            )
            .filter(|&(ex, ey)| ex >= 0 && ey >= 0 && ex < h as isize && ey < w as isize)
            .map(|(ex, ey)| (ex as usize, ey as usize))
            .filter(|&(ex, ey)| m[ex][ey] == b'.' || m[ex][ey] == b'E')
            // .filter(|&(ex, ey)| ex.abs_diff(wx) + ey.abs_diff(wy) <= 2) // part one
            .filter(|&(ex, ey)| ex.abs_diff(wx) + ey.abs_diff(wy) <= 20)
            {
                if let Some(e_to_t) = dis_to_t.get(&(ex, ey)) {
                    let tot = s_to_wall + wx.abs_diff(ex) + wy.abs_diff(ey) + e_to_t;
                    if tot + 100 <= baseline {
                        ans += 1;
                    }
                }
            }
        }
    }
    dbg!(ans);
}
