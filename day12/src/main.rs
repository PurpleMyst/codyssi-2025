use std::collections::VecDeque;

const SIDE: usize = 30;
const MODULO: i64 = 1_073_741_824;

fn main() {
    let input = include_str!("input.txt");
    let (grid_values, instructions_and_flow) = input.split_once("\n\n").unwrap();
    let (instructions, flow) = instructions_and_flow.split_once("\n\n").unwrap();

    let mut grid = [0i64; SIDE * SIDE];
    grid_values.split_ascii_whitespace().zip(grid.iter_mut()).for_each(|(value, cell)| {
        *cell = value.parse().unwrap();
    });

    solve_part1(instructions, grid);
    solve_part2(instructions, flow, grid);
    solve_part3(instructions, flow, grid);
}

fn solve_part1(instructions: &str, mut grid: [i64; SIDE*SIDE]) {
    for instruction in instructions.lines() {
        exec(instruction, &mut grid);
    }

    println!("{}", max_row_or_col_sum(grid));
}

fn max_row_or_col_sum(grid: [i64; SIDE*SIDE]) -> i64 {
    let mut part1 = i64::MIN;
    for i in 0..SIDE {
        let mut col_sum = 0;
        let mut row_sum = 0;
        for j in 0..SIDE {
            col_sum = col_sum + grid[j * SIDE + i];
            row_sum = row_sum + grid[i * SIDE + j];
        }
        part1 = part1.max(col_sum).max(row_sum);
    }
    part1
}

fn solve_part2(instructions: &str, flow: &str, mut grid: [i64; SIDE*SIDE]) {
    let mut instructions = instructions.lines().collect::<VecDeque<_>>();
    let mut queue = VecDeque::new();

    for action in flow.lines() {
        match action {
            "TAKE" => queue.push_back(instructions.pop_front().unwrap()),
            "CYCLE" => {
                instructions.push_back(queue.pop_front().unwrap());
            }
            "ACT" => {
                let instruction = queue.pop_front().unwrap();
                exec(instruction, &mut grid);
            }
            _ => unreachable!(),
        }
    }

    println!("{}", max_row_or_col_sum(grid));
}

fn solve_part3(instructions: &str, flow: &str, mut grid: [i64; SIDE*SIDE]) {
    let mut instructions = instructions.lines().collect::<VecDeque<_>>();
    let mut queue = VecDeque::new();

    'mainloop: loop {
    for action in flow.lines() {
        if instructions.is_empty() && queue.is_empty() {
            break 'mainloop;
        }
        match action {
            "TAKE" => queue.push_back(instructions.pop_front().unwrap()),
            "CYCLE" => {
                instructions.push_back(queue.pop_front().unwrap());
            }
            "ACT" => {
                let instruction = queue.pop_front().unwrap();
                exec(instruction, &mut grid);
            }
            _ => unreachable!(),
        }
    }
    }

    println!("{}", max_row_or_col_sum(grid));
}

fn exec(instruction: &str, grid: &mut [i64; SIDE * SIDE]) {
    let mut words = instruction.split(' ');
    match words.next().unwrap() {
        "SHIFT" => {
            let axis = words.next().unwrap();
            let idx = words.next().unwrap().parse::<usize>().unwrap() - 1;
            let _ = words.next().unwrap(); // "BY"
            let shift = words.next().unwrap().parse::<usize>().unwrap();

            match axis {
                "ROW" => grid[idx * SIDE..(idx + 1) * SIDE].rotate_right(shift),
                "COL" => {
                    let mut buffer = [0i64; SIDE];
                    grid.iter().skip(idx).step_by(SIDE).zip(buffer.iter_mut()).for_each(|(cell, buffer)| {
                        *buffer = *cell;
                    });
                    buffer.rotate_right(shift);
                    grid.iter_mut().skip(idx).step_by(SIDE).zip(buffer.iter_mut()).for_each(|(cell, buffer)| {
                        *cell = *buffer;
                    });
                }
                _ => unreachable!(),
            }
        }

        "ADD" => {
            let amount = words.next().unwrap().parse::<i64>().unwrap();
            match words.next().unwrap() {
                "ALL" => grid.iter_mut().for_each(|cell| {
                    *cell = (*cell + amount).rem_euclid(MODULO);
                }),
                "ROW" => {
                    let idx = words.next().unwrap().parse::<usize>().unwrap() - 1;
                    grid[idx * SIDE..(idx + 1) * SIDE].iter_mut().for_each(|cell| {
                        *cell = (*cell + amount).rem_euclid(MODULO);
                    });
                }
                "COL" => {
                    let idx = words.next().unwrap().parse::<usize>().unwrap() - 1;
                    grid.iter_mut().skip(idx).step_by(SIDE).for_each(|cell| {
                        *cell = (*cell + amount).rem_euclid(MODULO);
                    });
                }
                _ => unreachable!(),
            }
        }

        "SUB" => {
            let amount = words.next().unwrap().parse::<i64>().unwrap();
            match words.next().unwrap() {
                "ALL" => grid.iter_mut().for_each(|cell| {
                    *cell = (*cell - amount).rem_euclid(MODULO);
                }),
                "ROW" => {
                    let idx = words.next().unwrap().parse::<usize>().unwrap() - 1;
                    grid[idx * SIDE..(idx + 1) * SIDE].iter_mut().for_each(|cell| {
                        *cell = (*cell - amount).rem_euclid(MODULO);
                    });
                }
                "COL" => {
                    let idx = words.next().unwrap().parse::<usize>().unwrap() - 1;
                    grid.iter_mut().skip(idx).step_by(SIDE).for_each(|cell| {
                        *cell = (*cell - amount).rem_euclid(MODULO);
                    });
                }
                _ => unreachable!(),
            }
        }

        "MULTIPLY" => {
            let amount = words.next().unwrap().parse::<i64>().unwrap();
            match words.next().unwrap() {
                "ALL" => grid.iter_mut().for_each(|cell| {
                    *cell = (*cell * amount).rem_euclid(MODULO);
                }),
                "ROW" => {
                    let idx = words.next().unwrap().parse::<usize>().unwrap() -1;
                    grid[idx * SIDE..(idx + 1) * SIDE].iter_mut().for_each(|cell| {
                        *cell = (*cell * amount).rem_euclid(MODULO);
                    });
                }
                "COL" => {
                    let idx = words.next().unwrap().parse::<usize>().unwrap() - 1;
                    grid.iter_mut().skip(idx).step_by(SIDE).for_each(|cell| {
                        *cell = (*cell * amount).rem_euclid(MODULO);
                    });
                }
                _ => unreachable!(),
            }
        }

        _ => unreachable!(),
    }
}
