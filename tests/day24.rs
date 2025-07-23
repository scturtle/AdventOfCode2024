use ahash::{AHashMap, AHashSet};
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

#[derive(Debug, Clone, Copy)]
struct Rule<'a> {
    a: &'a str,
    b: &'a str,
    op: &'a str,
    out: &'a str,
}

impl<'a> Rule<'a> {
    fn execute(&self, a: i64, b: i64) -> i64 {
        match self.op {
            "OR" => a | b,
            "AND" => a & b,
            "XOR" => a ^ b,
            _ => unreachable!(),
        }
    }
}

impl<'a> std::fmt::Display for Rule<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {} -> {}", self.a, self.op, self.b, self.out)
    }
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

    // part one
    let tot = inits.len() + rules.len();
    let mut m: AHashMap<&str, i64> = inits.into_iter().collect();
    while m.len() < tot {
        for r in &rules {
            if !m.contains_key(&r.out) {
                if let (Some(&a), Some(&b)) = (m.get(&r.a), m.get(&r.b)) {
                    m.insert(r.out, r.execute(a, b));
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

    // part two
    let mut groups: Vec<Vec<Rule>> = vec![];
    let mut saw = AHashSet::new();

    for i in 0..45 {
        saw.insert(format!("x{i:02}"));
        saw.insert(format!("y{i:02}"));
        let mut group = vec![];
        for op in ["AND", "XOR", "AND", "OR"] {
            let mut changed = true;
            while changed {
                changed = false;
                for r in &rules {
                    if r.op == op && saw.contains(r.a) && saw.contains(r.b) && !saw.contains(r.out)
                    {
                        changed = true;
                        saw.insert(r.out.to_owned());
                        group.push(*r);
                    }
                }
            }
        }
        groups.push(group);
    }

    for g in &groups {
        for r in g {
            println!("{r}");
        }
        println!();
    }

    for i in 1..44 {
        let g = groups.get(i).unwrap();
        let ng = groups.get(i + 1).unwrap();
        if g[0].out != g[4].a && g[0].out != g[4].b {
            println!("group {}: {}", i, g[0]);
        }
        if (g[1].out != g[2].a && g[1].out != g[2].b) || (g[1].out != g[3].a && g[1].out != g[3].b)
        {
            println!("group {}: {}", i, g[1]);
        }
        if !g[2].out.starts_with('z') {
            println!("group {}: {}", i, g[2]);
        }
        if g[3].out != g[4].a && g[3].out != g[4].b {
            println!("group {}: {}", i, g[3]);
        }
        if (g[4].out != ng[2].a && g[4].out != ng[2].b)
            || (g[4].out != ng[3].a && g[4].out != ng[3].b)
        {
            println!("group {}: {}", i, g[4]);
        }
    }

    // some manual checks...
}
