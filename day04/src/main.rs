fn memory(c: u8) -> usize {
match c {
b'A'..=b'Z' => usize::from(c - b'A' + 1),
b'0'..=b'9' => usize::from(c - b'0'),
_ => 0,
}
}

fn rle(s: &str) -> String {
    let mut result = String::new();

    let mut n = 0;
    let mut last = b'\0';
    for c in s.bytes() {
        if c == last {
            n += 1;
        } else {
            if n > 0 {
                result.push_str(&format!("{}", n));
            }
            result.push(last as char);
            last = c;
            n = 1;
        }
    }
    if n > 0 {
        result.push_str(&format!("{}", n));
    result.push(last as char);
    }
    result
}

fn main() {
    let input = include_str!("input.txt");

    let part1 = input.bytes().map(memory).sum::<usize>();
    println!("{part1}");

    let part2 = input.lines().map(|line| {
        let keep = line.len() / 10;
        let start = &line[..keep];
        let end = &line[line.len() - keep..];
        let kept = line.len() - 2 * keep;
        format!("{start}{kept}{end}")
    }).map(|line| line.bytes().map(memory).sum::<usize>()).sum::<usize>();
    println!("{part2}");

    let part3 = input.lines().map(rle).map(|line| line.bytes().map(memory).sum::<usize>()).sum::<usize>();
    println!("{part3}");
}
