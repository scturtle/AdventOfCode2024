// use itertools::Itertools;
use nom::{
    IResult, Parser,
    character::complete::{char, i64, line_ending},
    multi::{many1, separated_list1 as sep1},
    sequence::separated_pair as sep_pair,
};

#[allow(clippy::type_complexity)]
fn parse(s: &str) -> IResult<&str, (Vec<(i64, i64)>, Vec<Vec<i64>>)> {
    let (s, rules) = sep1(line_ending, sep_pair(i64, char('|'), i64)).parse(s)?;
    let (s, _) = many1(line_ending).parse(s)?;
    let (s, updates) = sep1(line_ending, sep1(char(','), i64)).parse(s)?;
    Ok((s, (rules, updates)))
}

#[test]
fn day05() {
    // let txt = aoc::get_test_input().unwrap();
    let txt = aoc::get_input(5).unwrap();
    let (_, (rules, updates)) = parse(&txt).unwrap();
    let mut ans1 = 0;
    let mut ans2 = 0;
    for update in &updates {
        let mut ok = true;
        for (a, b) in &rules {
            let pos_a = update.iter().position(|x| x == a);
            let pos_b = update.iter().position(|x| x == b);
            if let (Some(pos_a), Some(pos_b)) = (pos_a, pos_b) {
                if pos_a > pos_b {
                    ok = false;
                }
            }
        }
        if ok {
            // part1
            ans1 += update[update.len() / 2];
        } else {
            // part2
            let mut update = update.to_vec();
            loop {
                let mut ok = true;
                for (a, b) in &rules {
                    let pos_a = update.iter().position(|x| x == a);
                    let pos_b = update.iter().position(|x| x == b);
                    if let (Some(pos_a), Some(pos_b)) = (pos_a, pos_b) {
                        if pos_a > pos_b {
                            update.swap(pos_a, pos_b);
                            ok = false;
                        }
                    }
                }
                if ok {
                    break;
                }
            }
            ans2 += update[update.len() / 2];
        }
    }
    dbg!(ans1);
    dbg!(ans2);
}
