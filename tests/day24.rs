use ahash::AHashMap;
use itertools::Itertools;
use nom::{
    IResult, Parser,
    bytes::{
        complete::{tag, take},
        is_a,
    },
    character::complete::{i64, line_ending, space1},
    multi::{many1, separated_list1 as sep1},
    sequence::separated_pair as sep_pair,
};

#[derive(Debug)]
enum Op {
    Or,
    And,
    Xor,
}

impl Op {
    fn execute(&self, a: i64, b: i64) -> i64 {
        match self {
            Op::Or => a | b,
            Op::And => a & b,
            Op::Xor => a ^ b,
        }
    }
}

#[derive(Debug)]
struct Rule<'a> {
    a: &'a str,
    b: &'a str,
    op: Op,
    out: &'a str,
}

fn parse_rule(s: &str) -> IResult<&str, Rule> {
    let (s, (a, _, op, _, b, _, out)) = (
        take(3usize),
        space1,
        is_a("ANDORX"),
        space1,
        take(3usize),
        tag(" -> "),
        take(3usize),
    )
        .parse(s)?;
    let op = match op {
        "AND" => Op::And,
        "OR" => Op::Or,
        "XOR" => Op::Xor,
        _ => unreachable!(),
    };
    Ok((s, Rule { a, b, op, out }))
}

#[allow(clippy::type_complexity)]
fn parse(s: &str) -> IResult<&str, (Vec<(&str, i64)>, Vec<Rule>)> {
    let (s, inits) = sep1(line_ending, sep_pair(take(3usize), tag(": "), i64)).parse(s)?;
    let (s, _) = many1(line_ending).parse(s)?;
    let (s, rules) = sep1(line_ending, parse_rule).parse(s)?;
    Ok((s, (inits, rules)))
}

#[test]
fn day24() {
    // let txt = aoc::get_test_input().unwrap();
    let txt = aoc::get_input(24).unwrap();
    let (_, (inits, rules)) = parse(&txt).unwrap();
    let tot = inits.len() + rules.len();
    let mut m: AHashMap<&str, i64> = inits.into_iter().collect();
    while m.len() < tot {
        for r in &rules {
            if !m.contains_key(&r.out) {
                if let (Some(&a), Some(&b)) = (m.get(&r.a), m.get(&r.b)) {
                    m.insert(r.out, r.op.execute(a, b));
                }
            }
        }
    }
    let ans1 = m
        .iter()
        .filter(|(k, _)| k.starts_with("z"))
        .sorted()
        .rev()
        .fold(0i64, |a, (_, b)| a * 2 + b);
    dbg!(ans1);
}
