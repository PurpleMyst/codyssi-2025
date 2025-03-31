use std::{cmp::Reverse, mem::swap, ops::Add};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Item {
    quality: usize,
    cost: usize,
    unique_materials: usize,
}

fn knapsack(items: &[Item], money: usize) -> usize {
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Debug)]
    struct Value {
        quality: usize,
        unique_materials: Reverse<usize>,
    }

    impl Add for Value {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Self {
                quality: self.quality + rhs.quality,
                unique_materials: Reverse(self.unique_materials.0 + rhs.unique_materials.0),
            }
        }
    }

    impl From<&Item> for Value {
        fn from(item: &Item) -> Self {
            Self {
                quality: item.quality,
                unique_materials: Reverse(item.unique_materials),
            }
        }
    }

    let mut prev = vec![Value::default(); money + 1];
    let mut next = vec![Value::default(); money + 1];

    for item in items {
        for j in 1..=money {
            if item.cost > j {
                next[j] = prev[j];
            } else {
                next[j] = prev[j].max(prev[j - item.cost] + Value::from(item));
            }
        }
        swap(&mut prev, &mut next);
    }

    let sol = prev[money];
    sol.quality * sol.unique_materials.0
}

fn main() {
    let input = include_str!("input.txt");

    let mut items = input
        .lines()
        .map(|line| {
            let mut it = line
                .split_once(" | ")
                .unwrap()
                .1
                .split(", ")
                .map(|part| part.split_once(": ").unwrap().1.parse().unwrap());

            (it.next().unwrap(), it.next().unwrap(), it.next().unwrap())
        })
        .map(|(quality, cost, unique_materials)| Item {
            quality,
            cost,
            unique_materials,
        })
        .collect::<Vec<_>>();

    items.sort_by_key(|item| (item.quality, item.cost));

    let part1 = items
        .iter()
        .rev()
        .take(5)
        .map(|item| item.unique_materials)
        .sum::<usize>();
    println!("{part1}");

    let part2 = knapsack(&items, 30);
    println!("{part2}");

    let part3 = knapsack(&items, 300);
    println!("{part3}");
}
