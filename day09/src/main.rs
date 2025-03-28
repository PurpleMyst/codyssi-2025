use std::cmp::min;

// With only 26 unique names, we use an array indexed by the first letter (A..Z)
const NUM_NAMES: usize = 26;

fn idx(name: &str) -> usize {
    // Assumes name starts with a unique capital letter A-Z.
    name.as_bytes()[0] as usize - b'A' as usize
}

fn parse_balance_line(line: &str) -> (usize, i16) {
    let (name, balance_str) = line.split_once(" HAS ").unwrap();
    let balance = balance_str.parse::<i16>().unwrap();
    (idx(name), balance)
}

fn parse_txn_line(line: &str) -> (usize, usize, i16) {
    let mut iter = line.split_whitespace().skip(1).step_by(2);
    let from = idx(iter.next().unwrap());
    let to = idx(iter.next().unwrap());
    let amt = iter.next().unwrap().parse::<i16>().unwrap();
    (from, to, amt)
}

fn sum_top_three(balances: &[i16; NUM_NAMES]) -> i16 {
    let mut top1 = i16::MIN;
    let mut top2 = i16::MIN;
    let mut top3 = i16::MIN;
    for &bal in balances.iter() {
        if bal > top1 {
            top3 = top2;
            top2 = top1;
            top1 = bal;
        } else if bal > top2 {
            top3 = top2;
            top2 = bal;
        } else if bal > top3 {
            top3 = bal;
        }
    }
    top1 + top2 + top3
}

fn solve_part1() -> i16 {
    let input = include_str!("input.txt");
    let (balances_str, transactions_str) = input.split_once("\n\n").unwrap();

    let mut state = [0i16; NUM_NAMES];
    for line in balances_str.lines() {
        let (i, bal) = parse_balance_line(line);
        state[i] = bal;
    }
    for line in transactions_str.lines() {
        let (from, to, amt) = parse_txn_line(line);
        state[from] -= amt;
        state[to] += amt;
    }
    sum_top_three(&state)
}

fn solve_part2() -> i16 {
    let input = include_str!("input.txt");
    let (balances_str, transactions_str) = input.split_once("\n\n").unwrap();

    let mut state = [0i16; NUM_NAMES];
    for line in balances_str.lines() {
        let (i, bal) = parse_balance_line(line);
        state[i] = bal;
    }
    for line in transactions_str.lines() {
        let (from, to, raw_amt) = parse_txn_line(line);
        let amt = min(raw_amt, state[from]);
        state[from] -= amt;
        state[to] += amt;
    }
    sum_top_three(&state)
}

fn solve_part3() -> i16 {
    let input = include_str!("input.txt");
    let (balances_str, transactions_str) = input.split_once("\n\n").unwrap();

    let mut state = [0i16; NUM_NAMES];
    for line in balances_str.lines() {
        let (i, bal) = parse_balance_line(line);
        state[i] = bal;
    }
    let mut debts: Vec<(usize, usize, i16)> = Vec::new();

    for line in transactions_str.lines() {
        let (from, to, mut amt) = parse_txn_line(line);
        if state[from] < amt {
            debts.push((from, to, amt - state[from]));
            amt = state[from];
        }
        state[from] -= amt;
        state[to] += amt;

        while let Some((debt_from, debt_to, debt_amt)) = debts.iter_mut().find(|(from, _, _)| state[*from] > 0) {
            let can_return = min(*debt_amt, state[*debt_from]);
            state[*debt_from] -= can_return;
            state[*debt_to] += can_return;
            *debt_amt -= can_return;
            debts.retain(|&(_, _, amt)| amt > 0);
        }
    }
    sum_top_three(&state)
}

fn main() {
    println!("{}", solve_part1());
    println!("{}", solve_part2());
    println!("{}", solve_part3());
}
