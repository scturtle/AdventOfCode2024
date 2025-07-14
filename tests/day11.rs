use nom::{
    IResult, Parser,
    character::complete::{i64, space1},
    multi::separated_list1 as sep1,
};

fn parse(s: &str) -> IResult<&str, Vec<i64>> {
    sep1(space1, i64).parse(s)
}

fn split(mut n: i64) -> Option<(i64, i64)> {
    let mut v = vec![];
    while n > 0 {
        v.push(n % 10);
        n /= 10;
    }
    let l = v.len();
    if l % 2 == 0 {
        let mut a = 0;
        for _ in 0..l / 2 {
            a = a * 10 + v.pop().unwrap();
        }
        let mut b = 0;
        for _ in 0..l / 2 {
            b = b * 10 + v.pop().unwrap();
        }
        Some((a, b))
    } else {
        None
    }
}

#[test]
fn day11() {
    // let txt = aoc::get_test_input().unwrap();
    let txt = aoc::get_input(11).unwrap();
    let (_, ns) = parse(&txt).unwrap();

    let mut v = ahash::AHashMap::<i64, i64>::new();
    for n in ns {
        *v.entry(n).or_default() += 1;
    }

    for i in 0..75 {
        let mut nv = ahash::AHashMap::new();
        for (n, c) in v {
            if n == 0 {
                *nv.entry(1).or_default() += c;
            } else if let Some((a, b)) = split(n) {
                *nv.entry(a).or_default() += c;
                *nv.entry(b).or_default() += c;
            } else {
                *nv.entry(2024 * n).or_default() += c;
            }
        }
        v = nv;
        if i == 24 {
            dbg!(v.values().sum::<i64>());
        }
    }
    dbg!(v.values().sum::<i64>());
}
