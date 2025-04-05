use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    str::FromStr,
};

use petgraph::{
    algo::{dijkstra, toposort},
    prelude::*,
};

#[derive(Clone, Copy)]
struct StaircaseName(u8);

#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
struct StaircaseStep(u8, u8);

impl FromStr for StaircaseName {
    type Err = <u8 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.strip_prefix("S").unwrap_or(s).parse()?))
    }
}

impl Display for StaircaseName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "S{}", self.0)
    }
}

impl Debug for StaircaseName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

impl FromStr for StaircaseStep {
    type Err = <u8 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split('_').map(|x| x.strip_prefix("S").unwrap_or(x).parse());
        Ok(Self(it.next().unwrap()?, it.next().unwrap()?))
    }
}

impl Display for StaircaseStep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "S{}_{}", self.0, self.1)
    }
}

impl Debug for StaircaseStep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

type StepGraph = DiGraphMap<StaircaseStep, u8>;

#[derive(Debug, Clone, Copy)]
struct Staircase {
    name: StaircaseName,
    start: usize,
    end: usize,
    feeding: StaircaseName,
    r#return: StaircaseName,
}

/// Load a problem statement, returning the N in S1_N and all the branching staircases.
fn load_input(input: &str) -> (usize, Vec<Staircase>) {
    let mut ls = input.lines();
    let l = ls.next().unwrap();
    let mut it = l.split_ascii_whitespace().skip(2).step_by(2);
    let start = it.next().unwrap().parse::<usize>().unwrap();
    debug_assert_eq!(start, 0);
    let end = it.next().unwrap().parse::<usize>().unwrap();

    let mut staircases = vec![];

    for l in ls.take_while(|l| !l.is_empty()) {
        let mut it = l.split_ascii_whitespace();
        let name = it.nth(0).unwrap();
        let start = it.nth(1).unwrap().parse::<usize>().unwrap();
        let end = it.nth(1).unwrap().parse::<usize>().unwrap();
        let feeding = it.nth(2).unwrap();
        let r#return = it.nth(1).unwrap();
        staircases.push(Staircase {
            name: name.parse().unwrap(),
            start,
            end,
            feeding: feeding.parse().unwrap(),
            r#return: r#return.parse().unwrap(),
        });
    }

    (end, staircases)
}

fn main() {
    let input = include_str!("input.txt");

    let mut ls = input.lines();
    let moves = ls
        .next_back()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(", ")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    debug_assert!(moves.contains(&1));

    let (end, staircases) = load_input(input);

    let mut paths = vec![0u128; end as usize + 1];
    paths[0] = 1;
    for i in 1..=end {
        for &j in &moves {
            if i >= j {
                paths[i] += paths[i - j];
            }
        }
    }

    let part1 = paths[end];
    println!("{part1}");

    let mut g = StepGraph::new();
    construct_graph(&mut g, end, &staircases);
    let part2 = solve_part2(&g, end, &moves);
    println!("{part2}");

    let k = part2.min(100000000000000000000000000000);
    let part3 = solve_for_kth_path(&g, end, &moves, k);
    let part3 = part3.unwrap().into_iter().map(|s| s.to_string()).collect::<Vec<_>>().join("-");
    println!("{part3}");
}

fn calculate_path_counts(
    g: &StepGraph,
    target: StaircaseStep,
) -> Result<HashMap<StaircaseStep, u128>, &'static str> {
    // Ensure the graph is a DAG and obtain a topological order of the nodes.
    // Note: toposort works on the graph directly.
    let topo_order = match toposort(g, None) {
         Ok(order) => order,
         Err(_) => return Err("Graph must be acyclic"), // Return an error if not a DAG
    };

    // DP table: maps each node to the number of paths from that node to the target.
    // Using standard HashMap here. FxHashMap is fine too.
    let mut path_counts: HashMap<StaircaseStep, u128> = HashMap::new();

    // Base case: there is exactly one path from the target to itself (the path of length 0).
    path_counts.insert(target, 1);

    // Process nodes in reverse topological order.
    for node in topo_order.into_iter().rev() {
        // Skip the target node itself if already processed (should be the first in reverse order)
        // Or compute its count (which will be 0 if it has neighbors, correctly).
        if node == target {
            // Already initialized
             continue;
        }

        // Calculate the sum of paths from neighbors that have already been processed
        // (which is guaranteed by reverse topological order).
        let mut current_count: u128 = 0;
        // Use graph's neighbors method
        for neighbor in g.neighbors(node) {
            // Get the count from the neighbor, default to 0 if neighbor hasn't been reached
            // or cannot reach the target. Check for overflow.
            let neighbor_count = path_counts.get(&neighbor).cloned().unwrap_or(0);
            current_count = current_count.checked_add(neighbor_count)
                .ok_or("Path count overflow detected")?; // Return error on overflow
        }
        path_counts.insert(node, current_count);

    }

    // Return the entire map of path counts.
    Ok(path_counts)
}

fn find_kth_path(
    g: &StepGraph,
    source: StaircaseStep,
    target: StaircaseStep,
    mut k: u128, // k is 1-based index
    path_counts: &HashMap<StaircaseStep, u128>,
) -> Option<Vec<StaircaseStep>> {

    // --- Input Validation ---
    let total_paths = path_counts.get(&source).cloned().unwrap_or(0);
    if k == 0 || k > total_paths {
        // k is 1-based, so k=0 is invalid.
        // If k > total_paths, the requested path doesn't exist.
        return None;
    }

    // If source is target, the only path is the node itself
    if source == target {
        return if k == 1 { Some(vec![source]) } else { None };
    }


    // --- Path Construction ---
    let mut path = vec![source];
    let mut current_node = source;

    while current_node != target {
        // Get neighbors of the current node
        let mut neighbors: Vec<_> = g.neighbors(current_node).collect();

        // Sort neighbors lexicographically (StaircaseStep derives Ord)
        neighbors.sort_unstable(); // Use unstable sort for potentially better performance

        let mut found_next_step = false;
        for &next_node in &neighbors {
            // Get the number of paths from this neighbor to the target
            let count_via_neighbor = path_counts.get(&next_node).cloned().unwrap_or(0);

            // Skip neighbors that cannot reach the target
            if count_via_neighbor == 0 {
                continue;
            }

            // Decision: Is the k-th path within the paths starting with next_node?
            if k <= count_via_neighbor {
                // Yes, this is the next node in our k-th path
                path.push(next_node);
                current_node = next_node;
                found_next_step = true;
                break; // Move to the next iteration of the outer while loop
            } else {
                // No, skip the paths going through this neighbor
                // Subtract the count of paths via this neighbor from k
                 if k > count_via_neighbor { // Ensure no underflow (although checked earlier)
                    k -= count_via_neighbor;
                 } else {
                     // This case should ideally not happen if total_paths check was correct
                     // and counts are accurate, but adding safety.
                     eprintln!("Warning: k became invalid during path construction.");
                     return None;
                 }

            }
        }

        // Safety check: If loop finishes without finding a next step, something is wrong
        // (e.g., graph structure changed, counts are inconsistent, or k was invalid).
        // This shouldn't happen in a correct DAG with valid k and counts.
        if !found_next_step && current_node != target {
             eprintln!("Error: Failed to find next step for k={} at node {:?}. Counts might be inconsistent.", k, current_node);
             return None; // Indicate error or inconsistency
        }
    }

    // If the loop terminates, we should have reached the target
    if current_node == target {
        Some(path)
    } else {
        // Should not happen if logic is correct
        None
    }
}

// Example of how to use the functions:
fn solve_for_kth_path(g: &StepGraph, end: usize, moves: &[usize], k: u128) -> Option<Vec<StaircaseStep>> {
    let contracted_g = contract_graph(g, moves);


    let start_node: StaircaseStep = format!("S1_0").parse().unwrap();
    let target_node: StaircaseStep = format!("S1_{}", end).parse().unwrap();

    // 1. Calculate all path counts from each node to the target
    match calculate_path_counts(&contracted_g, target_node) {
        Ok(path_counts) => {
            // Debug: Print total paths found

             // Check if target is reachable at all
             if !path_counts.contains_key(&start_node) || path_counts[&start_node] == 0 {
                 println!("Target {} is not reachable from {}", target_node, start_node);
                 return None;
             }

            // 2. Find the k-th path using the counts
            find_kth_path(&contracted_g, start_node, target_node, k, &path_counts)
        }
        Err(e) => {
            eprintln!("Error calculating path counts: {}", e);
            None
        }
    }
}

fn count_paths_dp(g: &StepGraph, source: StaircaseStep, target: StaircaseStep) -> u128 {
    // Ensure the graph is a DAG and obtain a topological order of the nodes.
    let topo_order = toposort(g, None).expect("Graph must be acyclic");

    // DP table: maps each node to the number of paths from that node to the target.
    let mut dp: rustc_hash::FxHashMap<StaircaseStep, u128> = Default::default();

    // Base case: there is exactly one path from the target to itself.
    dp.insert(target, 1);

    // Process nodes in reverse topological order.
    for node in topo_order.into_iter().rev() {
        // Skip the target (already set).
        if node == target {
            continue;
        }
        // Sum the number of paths from all neighbors of this node.
        let count = g.neighbors(node).map(|nbr| dp.get(&nbr).cloned().unwrap_or(0)).sum();
        dp.insert(node, count);
    }

    // Return the number of simple paths from the source.
    dp.get(&source).cloned().unwrap_or(0)
}

fn solve_part2(g: &StepGraph, end: usize, moves: &[usize]) -> u128 {
    let g = contract_graph(g, moves);
    let start = format!("{}_{}", "S1", 0).parse().unwrap();
    let target = format!("{}_{}", "S1", end).parse().unwrap();

    // Count the number of paths from the start to the target.
    count_paths_dp(&g, start, target)
}

fn contract_graph(g: &StepGraph, moves: &[usize]) -> StepGraph {
    let mut g2 = g.clone();
    g2.extend(g.nodes().flat_map(|n| {
        let moves = moves;
        dijkstra(g, n, None, |edge| *edge.weight())
            .into_iter()
            .filter_map(move |(m, d)| moves.contains(&(d as usize)).then_some((n, m, d)))
    }));
    g2
}

fn construct_graph(g: &mut StepGraph, end: usize, staircases: &[Staircase]) {
    for n in 0..end {
        // writeln!(f, "        {}_{} -> {}_{}", "S1", n, "S1", n + 1).unwrap();
        g.add_edge(
            format!("{}_{}", "S1", n).parse().unwrap(),
            format!("{}_{}", "S1", n + 1).parse().unwrap(),
            1,
        );
    }

    for staircase in staircases {
        for n in staircase.start..staircase.end {
            // writeln!(f, "        {}_{} -> {}_{}", staircase.name, n, staircase.name, n + 1).unwrap();
            g.add_edge(
                format!("{}_{}", staircase.name, n).parse().unwrap(),
                format!("{}_{}", staircase.name, n + 1).parse().unwrap(),
                1,
            );
        }
        g.add_edge(
            format!("{}_{}", staircase.feeding, staircase.start).parse().unwrap(),
            format!("{}_{}", staircase.name, staircase.start).parse().unwrap(),
            1,
        );
        g.add_edge(
            format!("{}_{}", staircase.name, staircase.end).parse().unwrap(),
            format!("{}_{}", staircase.r#return, staircase.end).parse().unwrap(),
            1,
        );
    }
}
