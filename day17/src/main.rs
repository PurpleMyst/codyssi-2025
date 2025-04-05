use petgraph::{
    algo::{dijkstra, toposort},
    prelude::*,
};
use rustc_hash::FxHashMap as HashMap;

mod input_parsing;
use input_parsing::{load_input, ProblemStatement, Staircase, StaircaseStep, StepGraph};

fn main() {
    let input = include_str!("input.txt");
    let ProblemStatement {
        s1_end: end,
        staircases,
        allowed_moves: moves,
    } = load_input(input);

    let part1 = solve_part1(end, &moves);
    println!("{part1}");

    let mut g = StepGraph::new();
    construct_graph(&mut g, end, &staircases);
    let g = contract_graph(&g, &moves);

    let start_step = StaircaseStep(1, 0);
    let end_step = StaircaseStep(1, end);

    let path_counts = calculate_path_counts(&g, end_step);

    let part2 = path_counts.get(&start_step).copied().unwrap_or(0);
    println!("{part2}");

    let k = part2.min(100_000_000_000_000_000_000_000_000_000);
    let part3 = find_kth_path(&g, start_step, end_step, k, &path_counts)
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
        .join("-");
    println!("{part3}");
}

fn solve_part1(end: u8, moves: &Vec<u8>) -> u128 {
    let mut paths = vec![0u128; usize::from(end + 1)];
    paths[0] = 1;
    for i in 1..=end {
        for &j in moves {
            if i >= j {
                paths[usize::from(i)] += paths[usize::from(i - j)];
            }
        }
    }
    paths[usize::from(end)]
}

fn calculate_path_counts(g: &StepGraph, target: StaircaseStep) -> HashMap<StaircaseStep, u128> {
    let topo_order = toposort(g, None).unwrap();

    let mut path_counts: HashMap<StaircaseStep, u128> = HashMap::default();

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
            let neighbor_count = path_counts.get(&neighbor).copied().unwrap_or(0);
            current_count += neighbor_count;
        }
        path_counts.insert(node, current_count);
    }

    // Return the entire map of path counts.
    path_counts
}

fn find_kth_path(
    g: &StepGraph,
    source: StaircaseStep,
    target: StaircaseStep,
    mut k: u128, // k is 1-based index
    path_counts: &HashMap<StaircaseStep, u128>,
) -> Vec<StaircaseStep> {
    // --- Input Validation ---
    let total_paths = path_counts.get(&source).copied().unwrap_or(0);
    debug_assert!(k != 0 && k <= total_paths);

    // --- Path Construction ---
    let mut path = vec![source];
    let mut current_node = source;

    let mut neighbors = Vec::new();

    'mainloop: while current_node != target {
        // Get neighbors of the current node
        neighbors.extend(g.neighbors(current_node));

        // Sort neighbors lexicographically (StaircaseStep derives Ord)
        neighbors.sort_unstable(); // Use unstable sort for potentially better performance

        for next_node in neighbors.drain(..) {
            // Get the number of paths from this neighbor to the target
            let count_via_neighbor = path_counts.get(&next_node).copied().unwrap_or(0);

            // Decision: Is the k-th path within the paths starting with next_node?
            if k <= count_via_neighbor {
                // Yes, this is the next node in our k-th path
                path.push(next_node);
                current_node = next_node;
                continue 'mainloop; // Move to the next iteration of the outer while loop
            } else {
                // No, skip the paths going through this neighbor
                // Subtract the count of paths via this neighbor from k
                k -= count_via_neighbor;
            }
        }

        // Safety check: If loop finishes without finding a next step, something is wrong
        // (e.g., graph structure changed, counts are inconsistent, or k was invalid).
        // This shouldn't happen in a correct DAG with valid k and counts.
        unreachable!();
    }

    // If the loop terminates, we should have reached the target
    path
}

fn contract_graph(g: &StepGraph, moves: &[u8]) -> StepGraph {
    use rayon::prelude::*;
    let mut g2 = g.clone();
    g2.extend(
        g.par_nodes()
            .flat_map_iter(|n| {
                dijkstra(g, n, None, |edge| *edge.weight())
                    .into_iter()
                    .filter_map(move |(m, d)| moves.contains(&d).then_some((n, m, d)))
            })
            .collect::<Vec<_>>(),
    );
    g2
}

fn construct_graph(g: &mut StepGraph, end: u8, staircases: &[Staircase]) {
    for n in 0..end {
        g.add_edge(StaircaseStep(1, n), StaircaseStep(1, n + 1), 1);
    }

    for staircase in staircases {
        for n in staircase.start..staircase.end {
            g.add_edge(
                StaircaseStep(staircase.name.0, n),
                StaircaseStep(staircase.name.0, n + 1),
                1,
            );
        }
        g.add_edge(
            StaircaseStep(staircase.feeding.0, staircase.start),
            StaircaseStep(staircase.name.0, staircase.start),
            1,
        );
        g.add_edge(
            StaircaseStep(staircase.name.0, staircase.end),
            StaircaseStep(staircase.returning.0, staircase.end),
            1,
        );
    }
}
