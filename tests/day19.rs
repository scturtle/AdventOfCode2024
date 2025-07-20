use itertools::Itertools;
use nom::{
    IResult, Parser,
    bytes::complete::{is_a, tag},
    character::complete::line_ending,
    multi::separated_list1 as sep1,
};

fn parse(s: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    let (s, pats) = sep1(tag(", "), is_a("wubrg")).parse(s)?;
    let (s, _) = (line_ending, line_ending).parse(s)?;
    let (s, targets) = sep1(line_ending, is_a("wubrg")).parse(s)?;
    Ok((s, (pats, targets)))
}

#[test]
fn day19() {
    // let txt = aoc::get_test_input().unwrap();
    let txt = aoc::get_input(19).unwrap();
    let (_, (pats, targets)) = parse(&txt).unwrap();
    let pats = pats.iter().map(|s| s.as_bytes()).collect_vec();
    let targets = targets.iter().map(|s| s.as_bytes()).collect_vec();

    let mut ans1 = 0;
    let mut ans2: i64 = 0;
    for target in targets {
        let n = target.len();
        let mut dp = vec![0; n + 1];
        dp[0] = 1;
        for i in 1..=n {
            for p in &pats {
                let m = p.len();
                if m <= i && target[i - m..i] == **p {
                    dp[i] += dp[i - m];
                }
            }
        }
        ans1 += (dp[n] > 0) as i64;
        ans2 += dp[n];
    }
    dbg!(ans1);
    dbg!(ans2);
}
