fn main() {
    let input = include_str!("input.txt");

    let mut ls = input.lines();
    let moves = ls.next_back().unwrap().split_once(": ").unwrap().1.split(", ").map(|x| {
        x.parse::<usize>().unwrap()
    }).collect::<Vec<_>>();

    let l = ls.next().unwrap();
    let mut it = l.split_ascii_whitespace().skip(2).step_by(2);
    let start = it.next().unwrap().parse::<usize>().unwrap();
    debug_assert_eq!(start, 0);
    let end = it.next().unwrap().parse::<usize>().unwrap();

    let mut paths = vec![0u128; end as usize + 1];

    paths[0] = 1;

    for i in 1..=end {
        for j in &moves {
            if i >= *j {
                paths[i] += paths[i - j];
            }
        }
    }

    let part1 = paths[end];
    println!("{part1}");
}
