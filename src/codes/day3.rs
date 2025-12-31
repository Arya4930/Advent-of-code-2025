use std::fs;

pub fn day3() {
    println!("Day 3");
    let contents = fs::read_to_string("src/codes/day3-inp.txt")
        .expect("Should have been able to read this file");
    let parts: Vec<&str> = contents.lines().collect();

    println!("Part 1: {}", part2(&parts, 2));
    println!("Part 2: {}", part2(&parts, 12));
    println!("---------------");
}

// old code
// fn part1(parts: &Vec<&str>) -> i64 {
//     let mut ctr = 0;

//     for part in parts {
//         let digits: Vec<i64> = part
//             .chars()
//             .map(|c| c.to_digit(10).unwrap() as i64)
//             .collect();

//         let mut max = 0;

//         for i in 0..digits.len() {
//             for j in i + 1..digits.len() {
//                 let val = digits[i] * 10 + digits[j];
//                 max = max.max(val);
//             }
//         }
//         ctr += max;
//     }

//     ctr
// }

fn part2(parts: &Vec<&str>, k: usize) -> i64 {
    let mut ctr = 0;

    for num in parts {
        let mut stack: Vec<char> = Vec::new();
        let mut to_remove = num.len() - k;

        for c in num.chars() {
            while to_remove > 0 && !stack.is_empty() && *stack.last().unwrap() < c {
                stack.pop();
                to_remove -= 1;
            }
            stack.push(c);
        }
        stack.truncate(k);
        ctr += stack.iter().collect::<String>().parse::<i64>().unwrap();
    }
    ctr
}
