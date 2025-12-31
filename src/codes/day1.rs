// Day 1

use std::fs;

pub fn day1() {
    println!("Day 1");
    let contents =
        fs::read_to_string("src/codes/day1-inp.txt")
            .expect("Should have been able to read this file");

    let parts: Vec<&str> = contents.lines().collect();
    println!("Part 1: {}", part1(&parts));
    println!("Part 2: {}", part2(&parts));
    println!("---------------");
}

fn part1(input: &Vec<&str>) -> i32 {
    let mut dial = 50;
    let mut ctr = 0;
    for part in input { 
        let c = part.chars().nth(0).unwrap();
        let val: i32 = part[1..].parse().unwrap();
        match c {
            'R' => dial = (dial + val) % 100,
            'L' => dial = (dial - val).rem_euclid(100),
            _ => unreachable!(),
        } 
        if dial == 0 { ctr += 1 }; 
    }
    ctr
}

fn part2(input: &Vec<&str>) -> i32 {
    let mut dial = 50;
    let mut ctr = 0;

    for part in input {
        let c = part.chars().nth(0).unwrap();
        let mut val: i32 = part[1..].parse().unwrap();
        if c == 'L' {
            val *= -1;
        }

        let prev = dial;
        dial += val;

        let mut crossings = (dial.div_euclid(100) - prev.div_euclid(100)).abs();
        dial = dial.rem_euclid(100);

        if c == 'L' {
            if prev == 0 {
                crossings -= 1;
            }
            if dial == 0 {
                crossings += 1;
            }
        }
        ctr += crossings;
    }
    ctr
}