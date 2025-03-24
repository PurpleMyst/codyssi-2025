fn reduce_part2(line: &str) -> usize {
    let mut bs = line.as_bytes().to_vec();
    while let Some(idx) = bs
        .windows(2)
        .position(|w| matches!(w, [b'0'..=b'9', b'a'..=b'z' | b'-']| [b'a'..=b'z' | b'-', b'0'..=b'9']))
    {
        bs.drain(idx..=idx+1);
    }
    bs.len()
}

fn reduce_part3(line: &str) -> usize {
    let mut bs = line.as_bytes().to_vec();
    while let Some(idx) = bs
        .windows(2)
        .position(|w| matches!(w, [b'0'..=b'9', b'a'..=b'z']| [b'a'..=b'z', b'0'..=b'9']))
    {
        bs.drain(idx..=idx+1);
    }
    bs.len()
}

fn main() {
    let input = include_str!("input.txt");

    let part1 = input.bytes().filter(|b| b.is_ascii_alphabetic()).count();
    println!("{part1}");

    let part2 = input.lines().map(reduce_part2).sum::<usize>();
    println!("{part2}");

    let part3 = input.lines().map(reduce_part3).sum::<usize>();
    println!("{part3}");
}
