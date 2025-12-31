use std::fs;

pub fn day4() {
    println!("Day 4");
    let contents = fs::read_to_string("src/codes/day4-inp.txt")
        .expect("Should have been able to read this file");
    let parts: Vec<&str> = contents.lines().collect();
    let mut grid: Vec<Vec<char>> = parts
        .iter()
        .map(|line| line.chars().collect())
        .collect();

    println!("Part 1: {}", part1(&grid).0);
    println!("Part 2: {}", part2(&mut grid));
    println!("---------------");
}

fn part1(grid: &Vec<Vec<char>>) -> (i32, Vec<(isize, isize)>) {
    let rows = grid.len();
    let mut store: Vec<(isize, isize)> = Vec::new();

    let mut ctr = 0;
    let dirs = [(-1, -1), (-1, 0), (0, -1), (1, 1), (1, 0), (0, 1), (-1, 1), (1, -1)];
    for i in 0..rows {
        let line = &grid[i];
        let cols  = grid[i].len();
        for j in 0..cols {
            let char =  line[j];
            if char == '@' {
                let mut inner_ctr = 0;
                
                for (dx, dy) in dirs {
                    let ni = i as isize + dx;
                    let nj = j as isize + dy;

                    if ni >= 0 && nj >= 0 && ni < rows as isize && nj < cols as isize {
                        if grid[ni as usize][nj as usize] == '@' {
                            inner_ctr += 1;
                        }
                    }
                }

                if inner_ctr < 4 {
                    ctr += 1;
                    store.push((i as isize, j as isize));
                }
            }
        }
    }

    (ctr, store)
}

fn part2(grid: &mut Vec<Vec<char>>) -> i32 {
    let mut ctr = 0;

    let mut fun = part1(grid);

    while fun.0 > 0 {
        ctr += fun.0;
        for &(dx, dy) in &fun.1 {
            grid[dx as usize][dy as usize] = '.';
        }
        fun = part1(&grid);
    }

    ctr
}