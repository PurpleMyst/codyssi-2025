use itertools::Itertools;

type Point = (i64, i64);

fn parse_tuple(s: &str) -> Point {
    s[1..s.len() - 1].split_once(", ").map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap())).unwrap()
}

fn dist(a: Point, b: Point) -> u64 {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn main() {
    let input = include_str!("input.txt");

    let (closest, furthest) = input.lines().map(parse_tuple).map(|p| dist((0, 0), p)).minmax().into_option().unwrap();
    let part1 = furthest - closest;
    println!("{part1}");

    let closest = input.lines().map(parse_tuple).min_by_key(|&p| dist((0, 0), p)).unwrap();
    let part2 = input.lines().map(parse_tuple).filter(|&p| p != closest).map(|p| dist(closest, p)).min().unwrap();
    println!("{part2}");

    let mut island = (0, 0);
    let mut islands = input.lines().map(parse_tuple).collect_vec();

    let mut part3 = 0;
    while let Some(next) = islands.iter().position_min_by_key(|&p| dist(island, *p)) {
        part3 += dist(island, islands[next]);
        island = islands.remove(next);
    }
    println!("{part3}");
}
