use std::collections::VecDeque;

const SIDE: usize = 50;

fn parse(input: &str) -> [u8; SIDE * SIDE] {
    let mut grid = [0; SIDE * SIDE];
    input
        .split_ascii_whitespace()
        .zip(grid.iter_mut())
        .for_each(|(value, cell)| *cell = value.parse().unwrap());

    grid
}

fn solve_part1(input: &str) -> impl std::fmt::Display {
    let grid = parse(input);
    let mut result = u64::MAX;

    for i in 0..SIDE {
        let mut row_danger = 0;
        let mut col_danger = 0;

        for j in 0..SIDE {
            row_danger += u64::from(grid[i * SIDE + j]);
            col_danger += u64::from(grid[j * SIDE + i]);
        }

        result = result.min(row_danger).min(col_danger);
    }

    result
}

fn solve_part2(input: &str) -> impl std::fmt::Display {
    let grid = parse(input);

    u64::from(grid[0])
        + pathfinding::prelude::dijkstra(
            &(0, 0),
            |&(x, y)| {
                [
                    ((x + 1) < 15).then(|| ((x + 1, y), u64::from(grid[(x + 1) * SIDE + y]))),
                    ((y + 1) < 15).then(|| ((x, y + 1), u64::from(grid[x * SIDE + y + 1]))),
                ]
                .into_iter()
                .flatten()
            },
            |&(x, y)| x == 14 && y == 14,
        )
        .unwrap()
        .1
}

fn solve_part3(input: &str) -> impl std::fmt::Display {
    let grid = parse(input);

    u64::from(grid[0])
        + pathfinding::prelude::dijkstra(
            &(0, 0),
            |&(x, y)| {
                [
                    ((x + 1) < SIDE).then(|| ((x + 1, y), u64::from(grid[(x + 1) * SIDE + y]))),
                    ((y + 1) < SIDE).then(|| ((x, y + 1), u64::from(grid[x * SIDE + y + 1]))),
                ]
                .into_iter()
                .flatten()
            },
            |&(x, y)| x == (SIDE - 1) && y == (SIDE - 1),
        )
        .unwrap()
        .1
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", solve_part1(input));
    println!("{}", solve_part2(input));
    println!("{}", solve_part3(input));
}
