fn main() {
    let input = include_str!("input.txt");
    let mut lines = input.lines();
    let signs = lines.next_back().unwrap();

    let numbers = lines.map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();

    let mut part1 = numbers[0];
    let mut part2 = numbers[0];

    for (sign, n) in signs.bytes().zip(numbers.iter().skip(1)) {
        match sign {
            b'+' => part1 += n,
            b'-' => part1 -= n,
            _ => unreachable!(),
        }
    }

    for (sign, n) in signs.bytes().rev().zip(numbers.iter().skip(1)) {
        match sign {
            b'+' => part2 += n,
            b'-' => part2 -= n,
            _ => unreachable!(),
        }
    }

    let mut part3 = numbers[0] * 10 + numbers[1];
    for (sign, n) in signs.bytes().rev().zip(numbers.chunks(2).skip(1)) {
        let n = n[0] * 10 + n[1];
        match sign {
            b'+' => part3 += n,
            b'-' => part3 -= n,
            _ => unreachable!(),
        }
    }

    println!("{part1}");
    println!("{part2}");
    println!("{part3}");
}
