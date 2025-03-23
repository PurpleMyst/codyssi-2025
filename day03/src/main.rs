use std::collections::HashSet;


fn parse_range(s: &str) -> (usize, usize) {
    let (start, end) = s.split_once('-').unwrap();
    (start.parse().unwrap(), end.parse().unwrap())
}

fn main() {
    let input = include_str!("input.txt");

    let part1 = input.split_ascii_whitespace()
        .map(|r| {
            let (start, end) = parse_range(r);
            (start..=end).count()
        })
    .sum::<usize>();

    let part2 = input.lines()
        .map(|l| {
            let (r1, r2) = l.split_once(' ').unwrap();
            let (start1, end1) = parse_range(r1);
            let (start2, end2) = parse_range(r2);

            let common_start = start1.max(start2);
            let common_end = end1.min(end2);

            (start1..=end1).count() + (start2..=end2).count() - (common_start..=common_end).count()
        })
    .sum::<usize>();

    let piles = input.lines()
        .map(|l| {
            let (r1, r2) = l.split_once(' ').unwrap();
            let (start1, end1) = parse_range(r1);
            let (start2, end2) = parse_range(r2);

            (start1..=end1).chain(start2..=end2).collect::<HashSet<_>>()
        })
    .collect::<Vec<_>>();

    let part3 = piles.windows(2)
        .map(|w| w[0].union(&w[1]).count())
        .max()
        .unwrap();

    println!("{part1}");
    println!("{part2}");
    println!("{part3}");
}
