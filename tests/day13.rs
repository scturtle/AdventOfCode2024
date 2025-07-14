use itertools::iproduct;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{i64, line_ending},
    multi::separated_list1 as sep1,
    sequence::{delimited, preceded},
};

#[allow(clippy::type_complexity)]
fn machine(s: &str) -> IResult<&str, ((i64, i64), (i64, i64), (i64, i64))> {
    let (s, ax) = preceded(tag("Button A: X+"), i64).parse(s)?;
    let (s, ay) = delimited(tag(", Y+"), i64, line_ending).parse(s)?;
    let (s, bx) = preceded(tag("Button B: X+"), i64).parse(s)?;
    let (s, by) = delimited(tag(", Y+"), i64, line_ending).parse(s)?;
    let (s, px) = preceded(tag("Prize: X="), i64).parse(s)?;
    let (s, py) = delimited(tag(", Y="), i64, line_ending).parse(s)?;
    Ok((s, ((ax, ay), (bx, by), (px, py))))
}

#[test]
fn day13() {
    // let txt = aoc::get_test_input().unwrap();
    let txt = aoc::get_input(13).unwrap();
    let (_, machines) = sep1(line_ending, machine).parse(&txt).unwrap();

    let mut ans1 = 0;
    for &((ax, ay), (bx, by), (px, py)) in &machines {
        if let Some(cost) = iproduct!(0..100, 0..100)
            .filter(|(i, j)| ax * i + bx * j == px && ay * i + by * j == py)
            .map(|(i, j)| 3 * i + j)
            .min()
        {
            ans1 += cost;
        }
    }
    dbg!(ans1);

    let mut ans2 = 0;
    let extra = 10000000000000;
    for &((ax, ay), (bx, by), (px, py)) in &machines {
        let (px, py) = (px + extra, py + extra);
        if ay * bx == ax * by {
            continue;
        }
        let xu = by * px - bx * py;
        let xd = ax * by - ay * bx;
        let yu = ay * px - ax * py;
        let yd = ay * bx - ax * by;
        if xu % xd != 0 || xu / xd < 0 || yu % yd != 0 || yu / yd < 0 {
            continue;
        }
        let (x, y) = (xu / xd, yu / yd);
        ans2 += x * 3 + y;
    }
    dbg!(ans2);
}
