#![allow(non_snake_case)]

use grid::Grid;

const SIDE: usize = 80;

const UP: usize = 0;
const FRONT: usize = 1;
const DOWN: usize = 2;
const BACK: usize = 3;
const LEFT: usize = 4;
const RIGHT: usize = 5;
const NUM_FACES: usize = 6;

fn clamp(n: u8) -> u8 {
    (n - 1) % 100 + 1
}

fn make_grid() -> Grid<u8> {
    Grid::init(SIDE, SIDE, 1)
}

fn dominant_sum(grid: &Grid<u8>) -> u16 {
    grid.iter_rows()
        .map(|row| row.map(|&x| x as u16).sum::<u16>())
        .chain(grid.iter_cols().map(|col| col.map(|&x| x as u16).sum::<u16>()))
        .max()
        .unwrap_or(0)
}

struct Cube {
    vertical_loop: [usize; 4],
    face_left: usize,
    face_right: usize,

    absorption: [u32; NUM_FACES],
    grids_part2: [Grid<u8>; NUM_FACES],
    grids_part3: [Grid<u8>; NUM_FACES],
}

impl Cube {
    fn new() -> Self {
        Cube {
            vertical_loop: [UP, FRONT, DOWN, BACK],
            face_left: LEFT,
            face_right: RIGHT,
            grids_part2: std::array::from_fn(|_| make_grid()),
            grids_part3: std::array::from_fn(|_| make_grid()),
            absorption: [0; NUM_FACES], // Initialize absorption array with zeros
        }
    }

    fn twist_U(&mut self) {
        self.vertical_loop.rotate_left(1);

        let left_idx = self.face_left;
        let right_idx = self.face_right;
        let apply_rotations = |grids: &mut [Grid<u8>; NUM_FACES]| {
            grids[left_idx].rotate_left();
            grids[right_idx].rotate_right();
        };
        apply_rotations(&mut self.grids_part2);
        apply_rotations(&mut self.grids_part3);
    }

    fn twist_L(&mut self) {
        let old_left = self.face_left;
        let old_right = self.face_right;
        let old_up = self.vertical_loop[UP];
        let old_down = self.vertical_loop[DOWN];
        self.face_left = old_up;
        self.face_right = old_down;
        self.vertical_loop[UP] = old_right;
        self.vertical_loop[DOWN] = old_left;

        let front_idx = self.vertical_loop[FRONT];
        let down_idx = self.vertical_loop[DOWN];
        let back_idx = self.vertical_loop[BACK];
        let right_idx = self.face_right;
        let apply_rotations = |grids: &mut [Grid<u8>; NUM_FACES]| {
            grids[front_idx].rotate_left();
            grids[right_idx].rotate_half();
            grids[down_idx].rotate_half();
            grids[back_idx].rotate_right();
        };
        apply_rotations(&mut self.grids_part2);
        apply_rotations(&mut self.grids_part3);
    }

    fn twist_D(&mut self) {
        for _ in 0..3 {
            self.twist_U();
        }
    }

    fn twist_R(&mut self) {
        for _ in 0..3 {
            self.twist_L();
        }
    }

    fn current_face(&self) -> usize {
        self.vertical_loop[UP]
    }

    fn apply_instruction(&mut self, instruction: &str) {
        let (target_str, value_str) = instruction.split_once(" - ").unwrap();

        let value: u8 = value_str.split_once(' ').unwrap().1.parse().unwrap();

        let current_face_idx = self.current_face();

        let (target_type, target_idx) = target_str.split_once(' ').unwrap_or_else(|| (target_str, "1"));
        let target_idx = target_idx.parse::<usize>().unwrap() - 1;

        let absorption_increment = value as u32
            * match target_type {
                "FACE" => (SIDE as u32).pow(2),
                "ROW" | "COL" => SIDE as u32,
                _ => unreachable!(),
            };
        self.absorption[current_face_idx] += absorption_increment;

        match target_type {
            "FACE" => {
                for val in self.grids_part2[current_face_idx].iter_mut() {
                    *val = clamp(*val + value);
                }
            }
            "COL" => {
                let x = target_idx;

                for val in self.grids_part2[current_face_idx].iter_col_mut(x) {
                    *val = clamp(*val + value);
                }
            }
            "ROW" => {
                let y = target_idx;

                for val in self.grids_part2[current_face_idx].iter_row_mut(y) {
                    *val = clamp(*val + value);
                }
            }
            _ => panic!("Unknown instruction target type for grid update: {}", target_type),
        }

        // --- Part 3: Grid Update with Twists (grids_part3) ---
        match target_type {
            "FACE" => {
                // No twists needed, just update the current face grid
                for val in self.grids_part3[current_face_idx].iter_mut() {
                    *val = clamp(*val + value);
                }
            }
            "COL" => {
                let x = target_idx;

                for _ in 0..4 {
                    for val in self.grids_part3[self.current_face()].iter_col_mut(x) {
                        *val = clamp(*val + value);
                    }
                    self.twist_U();
                }
            }
            "ROW" => {
                let y = target_idx;

                for _ in 0..4 {
                    for val in self.grids_part3[self.current_face()].iter_row_mut(y) {
                        *val = clamp(*val + value);
                    }
                    self.twist_R();
                }
            }

            _ => unreachable!(),
        }
    }

    fn apply_twist(&mut self, twist: u8) {
        match twist {
            b'U' => self.twist_U(),
            b'L' => self.twist_L(),
            b'D' => self.twist_D(),
            b'R' => self.twist_R(),
            _ => unreachable!(),
        }
    }
}

fn main() {
    let statement = include_str!("input.txt");

    let (instructions, twists) = statement
        .split_once("\n\n")
        .expect("Failed to split input into instructions and twists");

    let mut cube = Cube::new();

    let mut twists = twists.trim().bytes();

    for instruction in instructions.lines() {
        cube.apply_instruction(instruction);
        if let Some(twist) = twists.next() {
            cube.apply_twist(twist);
        }
    }

    let mut sorted_absorption = cube.absorption;
    sorted_absorption.sort_unstable();
    let part1: u64 = sorted_absorption.into_iter().rev().take(2).map(|n| n as u64).product();
    println!("{}", part1);

    let part2: u128 = cube.grids_part2.iter().map(|grid| dominant_sum(grid) as u128).product();
    println!("{}", part2);

    let part3: u128 = cube.grids_part3.iter().map(|grid| dominant_sum(grid) as u128).product();
    println!("{}", part3);
}
