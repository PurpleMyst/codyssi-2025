fn value(b: u8) -> u64 {
    u64::from(match b {
        b'a'..=b'z' => b - b'a' + 1,
        b'A'..=b'Z' => b - b'A' + 27,
        _ => 0,
    })
}

fn main() {
    let input = include_str!("input.txt").trim();

    let part1 = input.bytes().filter(|b| b.is_ascii_alphabetic()).count();
    println!("{part1}");

    let part2 = input.bytes().map(value).sum::<u64>();
    println!("{part2}");

    let part3 = input
        .bytes()
        .scan(0, |prev, b| {
            if b.is_ascii_alphabetic() {
                *prev = value(b);
            } else {
                let a = i64::try_from(2 * (*prev)).unwrap() - 5; // 1..=52
                let new_b = (a - 1).rem_euclid(52) + 1;
                *prev = new_b as u64;
            }
            Some(*prev)
        })
        .sum::<u64>();
    println!("{part3}");
}
