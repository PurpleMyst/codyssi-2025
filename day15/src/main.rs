#[derive(Clone, Copy, Debug)]
struct Tree {
    code: &'static str,
    id: u32,

    left: Option<usize>,
    right: Option<usize>,

    parent: Option<usize>,
}

type Arena = slab::Slab<Tree>;

impl Tree {
    fn new_leaf(code: &'static str, id: u32, parent: Option<usize>) -> Self {
        Self {
            code,
            id,
            left: None,
            right: None,
            parent,
        }
    }

    fn height(&self, arena: &Arena) -> usize {
        let left_height = self.left.map(|idx| arena[idx].height(arena)).unwrap_or(0);
        let right_height = self.right.map(|idx| arena[idx].height(arena)).unwrap_or(0);
        1 + left_height.max(right_height)
    }

    fn layer_sums(&self, arena: &Arena, layer: usize, layer_sums: &mut Vec<u32>) {
        layer_sums[layer] += self.id;

        if let Some(left) = self.left {
            arena[left].layer_sums(arena, layer + 1, layer_sums);
        }

        if let Some(right) = self.right {
            arena[right].layer_sums(arena, layer + 1, layer_sums);
        }
    }
}

fn parents(arena: &Arena, code: &'static str) -> Vec<usize> {
    let mut current = arena
        .iter()
        .find_map(|(idx, tree)| if tree.code == code { Some(idx) } else { None });
    std::iter::from_fn(move || {
        let idx = current?;
        current = arena[idx].parent;
        Some(idx)
    })
    .collect()
}

fn main() {
    let input = include_str!("input.txt");

    let mut lines = input.lines();
    let mut nodes = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(parse_node);

    let (root_id, root_value) = nodes.next().unwrap();
    let mut arena = Arena::new();
    let root_idx = arena.insert(Tree::new_leaf(root_id, root_value, None));

    for (code, id) in nodes {
        let mut current = root_idx;
        loop {
            if id < arena[current].id {
                if let Some(left) = arena[current].left {
                    current = left;
                } else {
                    arena[current].left = Some(arena.insert(Tree::new_leaf(code, id, Some(current))));
                    break;
                }
            } else if let Some(right) = arena[current].right {
                current = right;
            } else {
                arena[current].right = Some(arena.insert(Tree::new_leaf(code, id, Some(current))));
                break;
            }
        }
    }

    let h = arena[root_idx].height(&arena);
    let mut layer_sums = vec![0; h];
    arena[root_idx].layer_sums(&arena, 0, &mut layer_sums);
    let max_layer_sum = layer_sums.into_iter().max().unwrap();
    let part1 = max_layer_sum * h as u32;
    println!("{part1}");

    let part2_id = 500_000;
    let mut part2_path = Vec::new();
    let mut current = Some(root_idx);
    while let Some(current_val) = current {
        part2_path.push(arena[current_val].code);
        if part2_id < arena[current_val].id {
            current = arena[current_val].left;
        } else {
            current = arena[current_val].right;
        }
    }
    let part2 = part2_path.join("-");
    println!("{part2}");

    let (a_code, _) = parse_node(lines.next().unwrap());
    let (b_code, _) = parse_node(lines.next().unwrap());
    let a_parents = parents(&arena, a_code);
    let b_parents = parents(&arena, b_code);
    let part3 = arena[a_parents.into_iter().find(|parent| b_parents.contains(parent)).unwrap()].code;
    println!("{part3}");
}

fn parse_node(line: &str) -> (&str, u32) {
    let (id, value) = line.split_once(" | ").unwrap();
    (id, value.parse().unwrap())
}
