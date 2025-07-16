use itertools::Itertools;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{i64, line_ending},
    multi::separated_list1 as sep1,
    sequence::delimited,
};

fn parse(s: &str) -> IResult<&str, (i64, i64, i64, Vec<i64>)> {
    let (s, a) = delimited(tag("Register A: "), i64, line_ending).parse(s)?;
    let (s, b) = delimited(tag("Register B: "), i64, line_ending).parse(s)?;
    let (s, c) = delimited(tag("Register C: "), i64, line_ending).parse(s)?;
    let (s, _) = (line_ending, tag("Program: ")).parse(s)?;
    let (s, progs) = sep1(tag(","), i64).parse(s)?;
    Ok((s, (a, b, c, progs)))
}

#[test]
fn day17() {
    // let txt = aoc::get_test_input().unwrap();
    let txt = aoc::get_input(17).unwrap();
    let (_, (a, b, c, progs)) = parse(&txt).unwrap();

    let mut pc = 0;
    let mut a = a;
    let mut b = b;
    let mut c = c;
    let mut out = vec![];

    let com = |x, a, b, c| match x {
        0..=3 => x,
        4 => a,
        5 => b,
        6 => c,
        _ => unreachable!(),
    };

    while pc + 1 < progs.len() {
        let op = progs[pc];
        let opd = progs[pc + 1];
        let mut jump = false;
        match op {
            0 => {
                a = a >> com(opd, a, b, c);
            }
            1 => {
                b = b ^ opd;
            }
            2 => {
                b = com(opd, a, b, c) % 8;
            }
            3 => {
                if a != 0 {
                    pc = opd as usize;
                    jump = true;
                }
            }
            4 => {
                b = b ^ c;
            }
            5 => {
                out.push(com(opd, a, b, c) % 8);
            }
            6 => {
                b = a >> com(opd, a, b, c);
            }
            7 => {
                c = a >> com(opd, a, b, c);
            }
            _ => unreachable!(),
        }
        if !jump {
            pc += 2;
        }
    }
    let ans1 = out.iter().map(|x| x.to_string()).collect_vec().join(",");
    dbg!(ans1);

    // part two

    // while a != 0:
    //    print(((a ^ 1) ^ (a >> ((a & 7) ^ 2))) & 7, end=',')
    //    a >>= 3

    // so we try to infer the next 3 bits of a
    // from last number to first number in out

    let mut xs = vec![0];
    for &out in progs.iter().rev() {
        let mut nxs = vec![];
        for x in xs {
            for i in 0..8 {
                let a = x * 8 + i;
                let got = ((a ^ 1) ^ (a >> ((a & 7) ^ 2))) & 7;
                if got == out {
                    nxs.push(a);
                }
            }
        }
        xs = nxs;
    }
    dbg!(xs.iter().min().unwrap());
}
