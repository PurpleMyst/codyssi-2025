use std::fmt::Debug;

use fixedbitset::FixedBitSet;
use itertools::Itertools;
use pathfinding::prelude::*;
use rayon::prelude::*;

type N = i16;
type Vec3 = (N, N, N);
type Vec4 = (N, N, N, N);

const X_SIZE: N = 10;
const Y_SIZE: N = 15;
const Z_SIZE: N = 60;
const W_SIZE: N = 3;

const SPACE_SIZE_3D: usize = X_SIZE as usize * Y_SIZE as usize * Z_SIZE as usize;

const INITIAL_POSITION: Vec3 = (0, 0, 0);
const TARGET_POSITION: Vec3 = (9, 14, 59);

struct Rule {
    coeffs: Vec4,
    modulus: N,
    remainder: N,
    debris_velocity: Vec4,
}

struct DebrisPiece {
    initial_position: Vec4,
    velocity: Vec4,
}

impl DebrisPiece {
    fn position_at(&self, t: N) -> Vec4 {
        (
            (self.initial_position.0 + self.velocity.0 * t).rem_euclid(X_SIZE),
            (self.initial_position.1 + self.velocity.1 * t).rem_euclid(Y_SIZE),
            (self.initial_position.2 + self.velocity.2 * t).rem_euclid(Z_SIZE),
            ((self.initial_position.3 + self.velocity.3 * t) + 1).rem_euclid(W_SIZE) - 1,
        )
    }
}

fn vec4_to_vec3((x, y, z, w): Vec4) -> Option<Vec3> {
    (w == 0).then_some((x, y, z))
}

fn vec3_to_idx((x, y, z): Vec3) -> usize {
    (x * Y_SIZE * Z_SIZE) as usize + (y * Z_SIZE) as usize + z as usize
}

fn construct_bitset(debris: &[DebrisPiece], t: N) -> FixedBitSet {
    let mut bitset = FixedBitSet::with_capacity(SPACE_SIZE_3D);
    for piece in debris {
        if let Some(p) = vec4_to_vec3(piece.position_at(t)) {
            bitset.set(vec3_to_idx(p), true);
        }
    }
    bitset
}

fn construct_debris_map(debris: &[DebrisPiece], t: N) -> [u8; SPACE_SIZE_3D] {
    let mut debris_map = [0; SPACE_SIZE_3D];
    for piece in debris {
        if let Some(p) = vec4_to_vec3(piece.position_at(t)) {
            debris_map[vec3_to_idx(p)] += 1;
        }
    }
    debris_map
}

impl Debug for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}x + {}y + {}z + {}w DIVIDE {} HAS REMAINDER {} | DEBRIS VELOCITY ({}, {}, {}, {})",
            self.coeffs.0,
            self.coeffs.1,
            self.coeffs.2,
            self.coeffs.3,
            self.modulus,
            self.remainder,
            self.debris_velocity.0,
            self.debris_velocity.1,
            self.debris_velocity.2,
            self.debris_velocity.3
        )
    }
}

impl Rule {
    fn from_str(s: &str) -> Self {
        let parts: [_; 12] = s.splitn(12, ' ').collect_array().unwrap();
        let mut parts = parts.into_iter();

        let coeffs_str = parts.nth(2).unwrap();
        let coeffs = coeffs_str
            .split('+')
            .map(|c| c[..c.len() - 1].parse().unwrap())
            .next_tuple()
            .unwrap();

        let modulus = parts.nth(1).unwrap().parse().unwrap();
        let remainder = parts.nth(2).unwrap().parse().unwrap();

        let debris_velocity = parts
            .next_back()
            .unwrap()
            .strip_prefix('(')
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .split(", ")
            .map(|c| c.parse().unwrap())
            .next_tuple()
            .unwrap();

        Self {
            coeffs,
            modulus,
            remainder,
            debris_velocity,
        }
    }

    fn matches(&self, p: Vec4) -> bool {
        let (x, y, z, w) = p;
        let (a, b, c, d) = self.coeffs;

        let result = (a * x + b * y + c * z + d * w).rem_euclid(self.modulus);
        result == self.remainder
    }
}

fn feasible_space() -> impl Iterator<Item = Vec4> {
    itertools::iproduct!(0..X_SIZE, 0..Y_SIZE, 0..Z_SIZE, -W_SIZE / 2..=W_SIZE / 2)
}

fn in_bounds((x, y, z): Vec3) -> bool {
    (0..X_SIZE).contains(&x)
        && (0..Y_SIZE).contains(&y)
        && (0..Z_SIZE).contains(&z)
}

fn solve_part2<const MAX_T: N>(debris: &[DebrisPiece]) -> impl std::fmt::Display {
    let debris_positions = (0..MAX_T)
        .into_par_iter()
        .map(|t| construct_bitset(debris, t))
        .collect::<Vec<_>>();

    astar(
        &(INITIAL_POSITION, 0),
        |&((x, y, z), t)| {
            let debris_at_t = &debris_positions[t + 1];

            let neighbors = [
                (x, y, z),
                (x + 1, y, z),
                (x - 1, y, z),
                (x, y + 1, z),
                (x, y - 1, z),
                (x, y, z - 1),
                (x, y, z + 1),
            ];

            neighbors
                .into_iter()
                .filter(|&p| in_bounds(p))
                .filter(|&p| p == INITIAL_POSITION || !debris_at_t.contains(vec3_to_idx(p)))
                .filter_map(move |p| Some(((p, t + 1), 1)))
        },
        |&(p, _)| p.0.abs_diff(TARGET_POSITION.0) + p.1.abs_diff(TARGET_POSITION.1) + p.2.abs_diff(TARGET_POSITION.2),
        |&(p, _)| p == TARGET_POSITION,
    )
    .unwrap()
    .1
}

fn solve_part3<const MAX_T: N, const MAX_HP: u8>(debris: &[DebrisPiece]) -> impl std::fmt::Display {
    let debris_maps = (0..MAX_T)
        .into_par_iter()
        .map(|t| construct_debris_map(debris, t))
        .collect::<Vec<_>>();

    astar(
        &(INITIAL_POSITION, 0, MAX_HP),
        |&((x, y, z), t, hp)| {
            let debris_at_t = &debris_maps[t + 1];

            let neighbors = [
                (x, y, z),
                (x + 1, y, z),
                (x - 1, y, z),
                (x, y + 1, z),
                (x, y - 1, z),
                (x, y, z - 1),
                (x, y, z + 1),
            ];

            neighbors.into_iter().filter(|&p| in_bounds(p)).filter_map(move |p| {
                Some((
                    (
                        p,
                        t + 1,
                        if p == INITIAL_POSITION {
                            hp
                        } else {
                            hp.checked_sub(debris_at_t[vec3_to_idx(p)])?
                        },
                    ),
                    1,
                ))
            })
        },
        |&(p, _, _)| {
            p.0.abs_diff(TARGET_POSITION.0) + p.1.abs_diff(TARGET_POSITION.1) + p.2.abs_diff(TARGET_POSITION.2)
        },
        |&(p, _, _)| p == TARGET_POSITION,
    )
    .unwrap()
    .1
}

fn main() {
    let input = include_str!("input.txt");
    let rules = input.lines().map(Rule::from_str).collect_vec();

    let debris = rules
        .iter()
        .flat_map(|rule| {
            feasible_space().filter(|&p| rule.matches(p)).map(|p| DebrisPiece {
                initial_position: p,
                velocity: rule.debris_velocity,
            })
        })
        .collect_vec();
    println!("{}", debris.len());

    let (part2, part3) = rayon::join(|| solve_part2::<280>(&debris), || solve_part3::<220, 3>(&debris));
    println!("{part2}");
    println!("{part3}");
}
