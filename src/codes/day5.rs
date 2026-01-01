use std::fs;

pub fn day5() {
    println!("Day 5");
    let contents = fs::read_to_string("src/codes/day5-inp.txt")
        .expect("Should have been able to read this file");
    let (ranges_part, values_part) = contents.split_once("\r\n\r\n").unwrap();
    
    let mut ranges: Vec<(i64, i64)> = ranges_part
        .lines()
        .map(|line| {
            let (a,b) = line.split_once("-").unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();

    let values: Vec<i64> = values_part
        .lines()
        .map(|v| v.parse().unwrap())
        .collect();

    println!("Part 1: {}", part1(values, &ranges));
    println!("Part 2: {}", part2(&mut ranges));
    println!("---------------");
}

fn part1(values: Vec<i64>, ranges: &Vec<(i64, i64)>) -> i32 {
    let mut ctr = 0;

    for value in values {
        if ranges.iter().any(|&(l,r)| value >= l && value <= r) {
            ctr += 1;
        }
    }

    ctr
}

fn part2(ranges: &mut Vec<(i64, i64)>) -> i64 {
    let mut ctr = 0;
    
    ranges.sort_by_key(|r| r.0);

    let mut curr_start = ranges[0].0;
    let mut curr_end = ranges[0].1;

    for &(start, end) in &ranges[1..] {
        if start <= curr_end + 1 {
            curr_end = curr_end.max(end);
        } else {
            ctr += curr_end - curr_start + 1;
            curr_start = start;
            curr_end = end;
        }
    }

    ctr += curr_end - curr_start + 1;

    ctr
}