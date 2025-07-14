use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{i64, line_ending, space1},
    multi::separated_list1 as sep1,
    sequence::separated_pair as sep_pair,
};

fn parse(s: &str) -> IResult<&str, Vec<(i64, Vec<i64>)>> {
    sep1(line_ending, sep_pair(i64, tag(": "), sep1(space1, i64))).parse(s)
}

fn dfs(target: i64, ops: &[i64], i: usize, sofar: i64) -> bool {
    if i == ops.len() {
        return sofar == target;
    }
    dfs(target, ops, i + 1, sofar + ops[i]) || dfs(target, ops, i + 1, sofar * ops[i])
}

fn concat(a: i64, b: i64) -> i64 {
    let mut t = 1;
    while t <= b {
        t *= 10;
    }
    a * t + b
}

fn dfs2(target: i64, ops: &[i64], i: usize, sofar: i64) -> bool {
    if i == ops.len() {
        return sofar == target;
    }
    dfs2(target, ops, i + 1, concat(sofar, ops[i]))
        || dfs2(target, ops, i + 1, sofar + ops[i])
        || dfs2(target, ops, i + 1, sofar * ops[i])
}

#[test]
fn day07() {
    // let txt = aoc::get_test_input().unwrap();
    let txt = aoc::get_input(7).unwrap();
    let (_, eqs) = parse(&txt).unwrap();
    let ans1 = eqs
        .iter()
        .filter(|(target, ops)| dfs(*target, ops, 1, ops[0]))
        .map(|(target, _)| target)
        .sum::<i64>();
    dbg!(ans1);
    let ans2 = eqs
        .iter()
        .filter(|(target, ops)| dfs2(*target, ops, 1, ops[0]))
        .map(|(target, _)| target)
        .sum::<i64>();
    dbg!(ans2);
}
