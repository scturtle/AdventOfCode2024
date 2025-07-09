fn cnt(txt: &str) -> i64 {
    let pat = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut ans = 0;
    for cap in pat.captures_iter(txt) {
        ans += cap.get(1).unwrap().as_str().parse::<i64>().unwrap()
            * cap.get(2).unwrap().as_str().parse::<i64>().unwrap();
    }
    ans
}

#[test]
fn day03() {
    // let txt = aoc::get_test_input().unwrap();
    let txt = aoc::get_input(3).unwrap();
    let ans1 = cnt(&txt);
    dbg!(ans1);
    let mut ans2 = 0;
    for t in txt.split("do()") {
        let ts = t.split("don't()").collect::<Vec<_>>();
        ans2 += cnt(ts[0]);
    }
    dbg!(ans2);
}
