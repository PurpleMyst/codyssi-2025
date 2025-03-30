use std::{cmp::Reverse, ops::Add};

use scan_fmt::scan_fmt;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Item {
    quality: usize,
    cost: usize,
    unique_materials: usize,
}

fn knapsack(items: &'static [Item], money: usize) -> usize {
    let n = items.len();

    #[allow(non_snake_case)]
    let W = money;

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

    impl From<Item> for Value {
        fn from(item: Item) -> Self {
            Self {
                quality: item.quality,
                unique_materials: Reverse(item.unique_materials),
            }
        }
    }

    let mut m = vec![vec![Value::default(); W + 1]; n + 1];

    for i in 1..=n {
        for j in 1..=money {
            let item = items[i - 1];
            if item.cost > j {
                m[i][j] = m[i - 1][j];
            } else {
                m[i][j] = m[i - 1][j].max(m[i - 1][j - item.cost] + Value::from(item));
            }
        }
    }

    let sol = m[n][W];
    sol.quality * sol.unique_materials.0
}

fn main() {
    let input = include_str!("input.txt");

    let mut items = input
        .lines()
        .map(|line| {
            scan_fmt!(
                line,
                "{d} {} | Quality : {d}, Cost : {d}, Unique Materials : {d}",
                usize,
                String,
                usize,
                usize,
                usize
            )
            .unwrap()
        })
        .map(|(_id, _name, quality, cost, unique_materials)| Item {
            quality,
            cost,
            unique_materials,
        })
        .collect::<Vec<_>>();

    items.sort_by_key(|item| (item.quality, item.cost));

    let items = items.leak();

    let part1 = items
        .iter()
        .rev()
        .take(5)
        .map(|item| item.unique_materials)
        .sum::<usize>();
    println!("{part1}");

    let part2 = knapsack(items, 30);
    println!("{part2}");

    let part3 = knapsack(items, 300);
    println!("{part3}");
}
