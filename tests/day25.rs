use itertools::Itertools;
use nom::{
    IResult, Parser,
    bytes::complete::is_a,
    character::complete::{line_ending, multispace1},
    multi::separated_list1 as sep1,
};

fn parse(s: &str) -> IResult<&str, Vec<Vec<&str>>> {
    sep1(multispace1, sep1(line_ending, is_a("#."))).parse(s)
}

#[test]
fn day25() {
    // let txt = aoc::get_test_input().unwrap();
    let txt = aoc::get_input(25).unwrap();
    let (_, items) = parse(&txt).unwrap();

    let mut locks = vec![];
    let mut keys = vec![];
    for item in items {
        let cnts = (0..5)
            .map(|i| item.iter().filter(|r| r.as_bytes()[i] == b'#').count() - 1)
            .collect_vec();
        if item[0] == "#####" {
            locks.push(cnts);
        } else {
            keys.push(cnts);
        }
    }

    let mut ans = 0;
    for (l, k) in locks.iter().cartesian_product(keys.iter()) {
        if l.iter().zip(k.iter()).all(|(a, b)| a + b <= 5) {
            ans += 1;
        }
    }
    dbg!(ans);
}
