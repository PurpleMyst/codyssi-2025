use std::collections::HashMap;

fn solve_part1() -> i64 {
    let input = include_str!("input.txt");

    let (balances, transactions) = input.split_once("\n\n").unwrap();
    let mut state = balances
        .lines()
        .map(|line| {
            let (name, balance) = line.split_once(" HAS ").unwrap();
            let balance = balance.parse::<i64>().unwrap();
            (name, balance)
        })
        .collect::<HashMap<_, _>>();
    transactions.lines().for_each(|line| {
        let mut it = line.split(' ');
        let from = it.nth(1).unwrap();
        let to = it.nth(1).unwrap();
        let amt = it.nth(1).unwrap().parse::<i64>().unwrap();
        state.entry(from).and_modify(|bal| *bal -= amt);
        state.entry(to).and_modify(|bal| *bal += amt);
    });

    let mut m = state.values().copied().collect::<Vec<_>>();
    m.sort_unstable();
    m.into_iter().rev().take(3).sum::<i64>()
}

fn solve_part2() -> i64 {
    let input = include_str!("input.txt");

    let (balances, transactions) = input.split_once("\n\n").unwrap();
    let mut state = balances
        .lines()
        .map(|line| {
            let (name, balance) = line.split_once(" HAS ").unwrap();
            let balance = balance.parse::<i64>().unwrap();
            (name, balance)
        })
        .collect::<HashMap<_, _>>();
    transactions.lines().for_each(|line| {
        let mut it = line.split(' ');
        let from = it.nth(1).unwrap();
        let to = it.nth(1).unwrap();
        let amt = std::cmp::min(it.nth(1).unwrap().parse::<i64>().unwrap(), state[from]);
        state.entry(from).and_modify(|bal| *bal -= amt);
        state.entry(to).and_modify(|bal| *bal += amt);
    });

    let mut m = state.values().copied().collect::<Vec<_>>();
    m.sort_unstable();
    m.into_iter().rev().take(3).sum::<i64>()
}

fn solve_part3() -> i64 {
    let input = include_str!("input.txt");

    let (balances, transactions) = input.split_once("\n\n").unwrap();
    let mut balances = balances
        .lines()
        .map(|line| {
            let (name, balance) = line.split_once(" HAS ").unwrap();
            let balance = balance.parse::<i64>().unwrap();
            (name, balance)
        })
        .collect::<HashMap<_, _>>();
    let mut debts = Vec::new();
    transactions.lines().for_each(|line| {
        let mut it = line.split(' ');
        let from = it.nth(1).unwrap();
        let to = it.nth(1).unwrap();
        let mut amt = it.nth(1).unwrap().parse::<i64>().unwrap();

        if balances[from] < amt {
            debts.push((from, to, amt - balances[from]));
            amt = balances[from];
        }

        balances.entry(from).and_modify(|bal| *bal -= amt);
        balances.entry(to).and_modify(|bal| *bal += amt);

        while let Some((from, to, amt)) = debts.iter_mut().filter(|(from, _, _)| balances[from] > 0).next() {
            let can_return = std::cmp::min(*amt, balances[from]);
            balances.entry(from).and_modify(|bal| *bal -= can_return);
            balances.entry(to).and_modify(|bal| *bal += can_return);
            *amt -= can_return;

            debts.retain(|(_, _, amt)| *amt > 0);
        }
    });

    let mut m = balances.values().copied().collect::<Vec<_>>();
    m.sort_unstable();
    m.into_iter().rev().take(3).sum::<i64>()
}

fn main() {
    println!("{}", solve_part1());
    println!("{}", solve_part2());
    println!("{}", solve_part3());
}
