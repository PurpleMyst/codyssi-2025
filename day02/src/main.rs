fn price(quality: u64) -> u64 {
    quality.pow(3) * 69 + 510
}

fn main() {
    let input = include_str!("input.txt");
    let numbers = input
        .split_once("\n\n")
        .unwrap()
        .1
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let part1 = price(numbers[numbers.len() / 2]);
    println!("{part1}");

    let part2 = price(numbers.iter().filter(|&n| n % 2 == 0).sum());
    println!("{part2}");

    let part3 = numbers
        .iter()
        .filter(|&&n| price(n) < 15_000_000_000_000)
        .max()
        .unwrap();
    println!("{part3}");
}
