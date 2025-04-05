fn value(c: u8) -> u64 {
    match c {
        b'0'..=b'9' => u64::from(c - b'0'),
        b'A'..=b'Z' => u64::from(c - b'A' + 10),
        b'a'..=b'z' => u64::from(c - b'a' + 36),
        _ => unreachable!(),
    }
}

fn char(c: u8) -> u8 {
    match c {
        0..=9 => c + b'0',
        10..=35 => c - 10 + b'A',
        36..=61 => c - 36 + b'a',
        62 => b'!',
        63 => b'@',
        64 => b'#',
        65 => b'$',
        66 => b'%',
        67 => b'^',
        _ => unreachable!(),
    }
}

fn parse_line(line: &str) -> u64 {
    let (num, base) = line.split_once(' ').unwrap();
    let base = base.parse::<u64>().unwrap();
    num.bytes().fold(0, |acc, c| acc * base + value(c))
}

fn unparse(n: u64) -> String {
    let mut n = n;
    let mut res = Vec::new();
    while n > 0 {
        res.push(char((n % 68) as u8));
        n /= 68;
    }
    res.reverse();
    String::from_utf8(res).unwrap()
}

fn main() {
    let input = include_str!("input.txt");
    let numbers = input.lines().map(parse_line);
    let sum: u64 = numbers.clone().sum();

    let part1 = numbers.clone().max().unwrap();
    println!("{part1}");

    let part2 = unparse(sum);
    println!("{part2}");

    let part3 = (sum as f64).powf(0.25).ceil() as u64;
    println!("{part3}");
}
