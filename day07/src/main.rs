use itertools::Itertools;

fn main() {
    let (tracks, swaps, test_index) =
        include_str!("input.txt")
            .split("\n\n").collect_tuple().unwrap();

    let tracks = tracks.lines().collect_vec();
    let test_index = test_index.trim().parse::<usize>().unwrap();

    let swaps = swaps.lines().flat_map(|line| {
        let (a, b) = line.split_once('-').unwrap();
        [a.parse::<usize>().unwrap()-1, b.parse::<usize>().unwrap()-1]
    }).collect_vec();

    let mut part1_tracks = tracks.clone();
    for swap in swaps.chunks_exact(2) {
        part1_tracks.swap(swap[0], swap[1]);
    }
    let part1 = part1_tracks[test_index - 1];
    println!("{part1}");

    let mut part2_tracks = tracks.clone();
    for i in (0..swaps.len()).step_by(2) {
        let x = swaps[i];
        let y = swaps[i+1];
        let z = swaps[(i+2) % swaps.len()];

        let [x_val, y_val, z_val] = [part2_tracks[x], part2_tracks[y], part2_tracks[z]];
        part2_tracks[x] = z_val;
        part2_tracks[y] = x_val;
        part2_tracks[z] = y_val;
    }

    let part2 = part2_tracks[test_index - 1];
    println!("{part2}");

    let mut part3_tracks = tracks.clone();
    for swap in swaps.chunks_exact(2) {
        let &[x, y] = swap else { unreachable!(); };
        let [x, y] = [x.min(y), x.max(y)];

        let l = std::cmp::min(part3_tracks.len() - y, y - x);
        let block1 = x..x+l;
        let block2 = y..y+l;

        block1.zip(block2).for_each(|(i, j)| {
            part3_tracks.swap(i, j);
        });
    }
    let part3 = part3_tracks[test_index - 1];
    println!("{part3}");
}
