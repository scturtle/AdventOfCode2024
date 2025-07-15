use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{i64, line_ending},
    multi::separated_list1 as sep1,
    sequence::{preceded, separated_pair},
};

#[allow(clippy::type_complexity)]
fn robot(s: &str) -> IResult<&str, ((i64, i64), (i64, i64))> {
    let (s, p) = preceded(tag("p="), separated_pair(i64, tag(","), i64)).parse(s)?;
    let (s, v) = preceded(tag(" v="), separated_pair(i64, tag(","), i64)).parse(s)?;
    Ok((s, (p, v)))
}

#[test]
fn day14() {
    // let txt = aoc::get_test_input().unwrap();
    // let (w, h) = (11, 7);
    let txt = aoc::get_input(14).unwrap();
    let (w, h) = (101, 103);
    let (_, robots) = sep1(line_ending, robot).parse(&txt).unwrap();

    let mut rs = robots.clone();
    for _ in 0..100 {
        for r in &mut rs {
            r.0 = ((r.0.0 + r.1.0).rem_euclid(w), (r.0.1 + r.1.1).rem_euclid(h));
        }
    }
    let mut cnt = [0, 0, 0, 0];
    for ((x, y), _) in rs {
        cnt[0] += if x < w / 2 && y < h / 2 { 1 } else { 0 };
        cnt[1] += if x > w / 2 && y < h / 2 { 1 } else { 0 };
        cnt[2] += if x < w / 2 && y > h / 2 { 1 } else { 0 };
        cnt[3] += if x > w / 2 && y > h / 2 { 1 } else { 0 };
    }
    let ans1 = cnt[0] * cnt[1] * cnt[2] * cnt[3];
    dbg!(ans1);

    let mut rs = robots.clone();
    for sec in 1..10000 + 1 {
        for r in &mut rs {
            r.0 = ((r.0.0 + r.1.0).rem_euclid(w), (r.0.1 + r.1.1).rem_euclid(h));
        }
        let cnt = rs
            .iter()
            .filter(|((x, y), _)| *x >= 40 && *x <= 70 && *y >= 40 && *y <= 80)
            .count();
        if cnt >= 300 {
            dbg!(sec);
        }
        //     let mut m = vec![vec!['.'; w as usize]; h as usize];
        //     for r in &rs { m[r.0.1 as usize][r.0.0 as usize] = '*'; }
        //     println!("{}", m.into_iter().map(|v| v.into_iter().collect::<String>()).collect_vec().join("\n"));
        //     std::thread::sleep(std::time::Duration::from_millis(10));
        // }
    }
}
