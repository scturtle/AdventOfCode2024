use itertools::Itertools;
use nom::{
    IResult, Parser,
    character::{
        complete::{i64, line_ending},
        streaming::space1,
    },
    multi::separated_list1 as sep1,
};

fn parse(input: &str) -> IResult<&str, Vec<Vec<i64>>> {
    sep1(line_ending, sep1(space1, i64)).parse(input)
}

fn is_safe(r: &[i64]) -> bool {
    (r.iter().tuple_windows().all(|(a, b)| a < b) || r.iter().tuple_windows().all(|(a, b)| a > b))
        && r.iter().tuple_windows().all(|(a, b)| (a - b).abs() <= 3)
}

fn try_del1(r: &[i64]) -> bool {
    (0..r.len()).any(|i| {
        is_safe(
            r.iter()
                .cloned()
                .take(i)
                .chain(r.iter().cloned().dropping(i + 1))
                .collect::<Vec<_>>()
                .as_slice(),
        )
    })
}

#[test]
fn day02() {
    // let txt = aoc::get_test_input().unwrap();
    let txt = aoc::get_input(2).unwrap();
    let (_, m) = parse(&txt).unwrap();
    let ans1 = m.iter().filter(|v| is_safe(v)).count();
    dbg!(ans1);
    let ans2 = m.iter().filter(|v| is_safe(v) || try_del1(v)).count();
    dbg!(ans2);
}
