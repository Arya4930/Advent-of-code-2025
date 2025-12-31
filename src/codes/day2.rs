use std::fs;

pub fn day2() {
    println!("Day 2");
    let contents = fs::read_to_string("src/codes/day2-inp.txt")
            .expect("Should have been able to read this file");

    let ranges: Vec<&str> = contents.split(",").collect();
    let (p1, p2) = calculations(ranges);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    println!("---------------");
}

fn calculations(ranges: Vec<&str>) -> (i64, i64) {
    let mut ctr1 = 0;
    let mut ctr2 = 0;

    for range in ranges {
        let r: Vec<i64> = range
            .split("-")
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        for i in r[0]..=r[1] {
            let res = is_invalid_id(i);
            if res.0 {
                ctr2 += i;
                if i.to_string().len() / res.1 == 2 {
                    ctr1 += i;
                }
            }
        }
    }
    (ctr1, ctr2)
}

fn is_invalid_id(n: i64) -> (bool, usize) {
    let s = n.to_string();
    let len = s.len();

    for k in 1..=len/2 {
        if len % k != 0 {
            continue;
        }

        let pattern = &s[0..k];
        let repeated = pattern.repeat(len / k);

        if repeated == s {
            return (true, k);
        }
    }

    (false, 0)
}