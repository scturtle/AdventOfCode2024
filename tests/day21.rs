use ahash::AHashMap;
use itertools::{Itertools, iproduct};
use std::iter::repeat_n;

struct Keyboard {
    ch_to_pos: AHashMap<char, (isize, isize)>,
    invalid: (isize, isize),
}

impl Keyboard {
    fn new(kbd: Vec<Vec<char>>) -> Self {
        let mut ch_to_pos = AHashMap::new();
        for (i, j) in iproduct!(0..kbd.len(), 0..kbd[0].len()) {
            ch_to_pos.insert(kbd[i][j], (i as isize, j as isize));
        }
        let invalid = ch_to_pos[&' '];
        Self { ch_to_pos, invalid }
    }
    fn pos_of(&self, c: char) -> (isize, isize) {
        *self.ch_to_pos.get(&c).unwrap()
    }
}

fn check(cur: (isize, isize), moves: &[char], invalid: (isize, isize)) -> bool {
    let mut cur = cur;
    for c in moves {
        match c {
            '^' => {
                cur.0 -= 1;
            }
            'v' => {
                cur.0 += 1;
            }
            '<' => {
                cur.1 -= 1;
            }
            '>' => {
                cur.1 += 1;
            }
            _ => unreachable!(),
        }
        if cur == invalid {
            return false;
        }
    }
    true
}

fn dfs(
    a: char,
    b: char,
    kbd0: &Keyboard,
    kbd1: &Keyboard,
    level: usize,
    max_level: usize,
    cache: &mut AHashMap<(char, char, usize), usize>,
) -> usize {
    if let Some(&l) = cache.get(&(a, b, level)) {
        return l;
    }
    let kbd = if level == 0 { kbd0 } else { kbd1 };
    let cur = kbd.pos_of(a);
    let nxt = kbd.pos_of(b);
    let mut steps = vec![];
    if nxt.1 - cur.1 < 0 {
        steps.extend(repeat_n('<', (cur.1 - nxt.1) as usize));
    }
    if nxt.1 - cur.1 > 0 {
        steps.extend(repeat_n('>', (nxt.1 - cur.1) as usize));
    }
    if nxt.0 - cur.0 < 0 {
        steps.extend(repeat_n('^', (cur.0 - nxt.0) as usize));
    }
    if nxt.0 - cur.0 > 0 {
        steps.extend(repeat_n('v', (nxt.0 - cur.0) as usize));
    }
    let k = steps.len();
    let steps = steps
        .into_iter()
        .permutations(k)
        .unique()
        .filter(|x| check(cur, x, kbd.invalid))
        .collect_vec();
    let l = if level == max_level {
        steps.iter().map(|x| x.len() + 1).min().unwrap()
    } else {
        steps
            .iter()
            .map(|step| {
                let mut a = 'A';
                let mut cnt = 0;
                for &b in step {
                    cnt += dfs(a, b, kbd0, kbd1, level + 1, max_level, cache);
                    a = b;
                }
                cnt += dfs(a, 'A', kbd0, kbd1, level + 1, max_level, cache);
                cnt
            })
            .min()
            .unwrap()
    };
    cache.insert((a, b, level), l);
    l
}

#[test]
fn day21() {
    // let txt = aoc::get_test_input().unwrap();
    let txt = aoc::get_input(21).unwrap();

    let codes = txt.lines().collect_vec();
    let kbd0 = Keyboard::new(vec![
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec![' ', '0', 'A'],
    ]);
    let kbd1 = Keyboard::new(vec![vec![' ', '^', 'A'], vec!['<', 'v', '>']]);

    // let max_level = 2; // part one
    let max_level = 25; // part two

    let mut ans = 0;
    let mut cache = AHashMap::new();
    for code in codes {
        let mut a = 'A';
        let mut l = 0;
        for b in code.chars() {
            l += dfs(a, b, &kbd0, &kbd1, 0, max_level, &mut cache);
            a = b;
        }
        let n: usize = code[..3].parse().unwrap();
        ans += l * n;
    }
    dbg!(ans);
}
