use nom::{
    IResult, Parser,
    character::{
        complete::{i64, line_ending},
        streaming::space1,
    },
    multi::separated_list1,
    sequence::separated_pair,
};

fn parse(s: &str) -> IResult<&str, Vec<(i64, i64)>> {
    separated_list1(line_ending, separated_pair(i64, space1, i64)).parse(s)
}

#[test]
fn day01() {
    // let txt = aoc::get_test_input().unwrap();
    let txt = aoc::get_input(1).unwrap();
    let (_, v) = parse(&txt).unwrap();
    let mut va: Vec<_> = v.iter().map(|(a, _)| a).collect();
    let mut vb: Vec<_> = v.iter().map(|(_, b)| b).collect();
    va.sort_unstable();
    vb.sort_unstable();
    let ans1 = va
        .iter()
        .zip(vb.iter())
        .map(|(&&a, &&b)| (a - b).abs())
        .sum::<i64>();
    dbg!(ans1);
    let ans2 = va
        .iter()
        .map(|a| vb.iter().filter(|&b| a == b).count() as i64 * **a)
        .sum::<i64>();
    dbg!(ans2);
}
