use std::collections::{HashMap, HashSet};

use pathfinding::prelude::*;

fn longest_cycle_starting_from(map: &HashMap<&str, Vec<(&str, u64)>>, start: &str) -> u64 {
    let mut q = vec![(start, 0, HashSet::new())];

    let mut solution = u64::MIN;

    while let Some((current, cost, visited)) = q.pop() {
        for (next, next_cost) in map.get(current).unwrap() {
            if *next == start {
                solution = solution.max(cost + next_cost);
                continue;
            }

            let mut next_visited = visited.clone();
            if next_visited.insert(next) {
                q.push((next, cost + next_cost, next_visited));
            }
        }
    }

    solution
}

fn main() {
    let input = include_str!("input.txt");

    let mut paths = HashMap::new();
    input.lines().for_each(|line| {
        let (edge, weight) = line.split_once(" | ").unwrap();
        let (from, to) = edge.split_once(" -> ").unwrap();
        paths
            .entry(from)
            .or_insert_with(Vec::new)
            .push((to, weight.parse::<u64>().unwrap()));
    });

    let part1_paths = dijkstra_all(&"STT", |current| {
        paths.get(current).unwrap().iter().map(|(next, _)| (*next, 1))
    });
    let mut part1_costs = part1_paths.values().map(|(_, c)| c).collect::<Vec<_>>();
    part1_costs.sort_unstable();
    let part1 = part1_costs.into_iter().rev().take(3).product::<u64>();
    println!("{part1}");

    let part2_paths = dijkstra_all(&"STT", |current| paths.get(current).unwrap().iter().copied());
    let mut part2_costs = part2_paths.values().map(|(_, c)| c).collect::<Vec<_>>();
    part2_costs.sort_unstable();
    let part2 = part2_costs.into_iter().rev().take(3).product::<u64>();
    println!("{part2}");

    let part3 = paths
        .keys()
        .map(|node| longest_cycle_starting_from(&paths, node))
        .max()
        .unwrap();
    println!("{part3}");
}
