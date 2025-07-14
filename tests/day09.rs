#[test]
fn day09() {
    // let txt = aoc::get_test_input().unwrap();
    let txt = aoc::get_input(9).unwrap();
    let input = txt.trim().as_bytes();
    let mut disk = vec![];
    let mut is_free = false;
    let mut id: isize = 0;
    for &c in input {
        let c = c - b'0';
        for _ in 0..c {
            disk.push(if is_free { -1 } else { id });
        }
        if !is_free {
            id += 1;
        }
        is_free = !is_free;
    }
    let bak = disk.clone();

    // part one
    for i in (0..disk.len()).rev() {
        if disk[i] != -1 {
            for j in 0..i {
                if disk[j] == -1 {
                    disk[j] = disk[i];
                    disk[i] = -1;
                    break;
                }
            }
        }
    }
    let ans1 = disk
        .iter()
        .enumerate()
        .map(|(i, &c)| if c == -1 { 0 } else { i * c as usize })
        .sum::<usize>();
    dbg!(ans1);

    // part two
    disk = bak;
    let mut i = disk.len() as isize - 1;
    let mut next_id = id - 1;
    while i >= 0 {
        if disk[i as usize] != next_id {
            i -= 1;
            continue;
        }
        let j = i;
        while i > 0 && disk[i as usize - 1] == next_id {
            i -= 1;
        }
        let cnt = j - i + 1;
        let mut free_cnt = 0;
        let mut found = None;
        for k in 0..i {
            if disk[k as usize] == -1 {
                free_cnt += 1;
            } else {
                free_cnt = 0;
            }
            if free_cnt == cnt {
                found = Some(k - cnt + 1);
                break;
            }
        }
        if let Some(k) = found {
            for t in 0..cnt as usize {
                disk[k as usize + t] = next_id;
                disk[i as usize + t] = -1;
            }
        }
        i -= 1;
        next_id -= 1;
    }
    let ans2 = disk
        .iter()
        .enumerate()
        .map(|(i, &c)| if c == -1 { 0 } else { i * c as usize })
        .sum::<usize>();
    dbg!(ans2);
}
