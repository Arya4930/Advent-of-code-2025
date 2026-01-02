use std::fs;

pub fn day6() {
    println!("Day 6");
    let contents = fs::read_to_string("src/codes/day6-inp.txt")
            .expect("Should have been able to read this file");
    
    let parts: Vec<&str> = contents.lines().collect();
    let grid: Vec<Vec<char>> = parts
        .iter()
        .map(|line| line.chars().collect())
        .collect();
    
    let rows = grid.len();
    let cols = grid[0].len();

    let ops: Vec<char> = grid[rows - 1]
        .iter()
        .filter(|&&c| c != ' ')
        .copied()
        .collect();

    let mut num_grid: Vec<Vec<i64>> = Vec::new();
    let mut ctr = 0;

    for i in 0..(rows - 1) {
        let mut row_nums = Vec::new();
        let mut start = 0;
        while start < cols {
            while start < cols && grid[i][start] == ' ' {
                start += 1;
            }

            if start >= cols {
                break;
            }

            let mut end = start;
            while end < cols && grid[i][end] != ' ' {
                end += 1;
            }

            let num_str: String = grid[i][start..end].iter().collect();
            let num = num_str.parse::<i64>().unwrap();
            row_nums.push(num);
            start = end;
        }
        num_grid.push(row_nums);
    }

    for i in 0..ops.len() {
        let op = ops[i];
        let mut temp = if op == '*' { 1 } else { 0 };

        for row in 0..num_grid.len() {
            let num = num_grid[row][i];
            if op == '*' {
                temp *= num;
            } else {
                temp += num;
            }
        }
        ctr += temp;
    }
    
    println!("Part 1: {}", ctr);
    // println!("Part 2: {}", part2(&parts));
    println!("---------------");
}