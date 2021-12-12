use aoc2021::get_input;
use std::collections::HashMap;

#[derive(Debug, Default)]
struct Node {
    links: Vec<i32>,
    is_big: bool,
}

#[derive(Default)]
struct IdTable<'a> {
    reverse_lables: HashMap<&'a str, i32>,
    next_id: i32,
}

impl<'a> IdTable<'a> {
    fn add(&mut self, s: &'a str) -> i32 {
        *self.reverse_lables.entry(s).or_insert_with(|| {
            let id = self.next_id;
            self.next_id += 1;
            id
        })
    }
}

enum Command {
    Push(i32),
    Pop(i32),
}

fn solve(input: &str) -> i32 {
    let mut ids = IdTable::default();
    let mut graph = Vec::new();

    for line in input.lines() {
        let mut parts = line.split('-');
        let from = parts.next().unwrap();
        let to = parts.next().unwrap();

        let from_id = ids.add(from);
        let to_id = ids.add(to);

        let max_id = from_id.max(to_id);

        if max_id >= graph.len() as i32 {
            graph.resize_with((max_id + 1) as usize, Node::default);
        }

        {
            let node = &mut graph[from_id as usize];
            node.links.push(to_id);
            node.is_big = from.chars().next().unwrap().is_uppercase();
        }
        {
            let node = &mut graph[to_id as usize];
            node.links.push(from_id);
            node.is_big = to.chars().next().unwrap().is_uppercase();
        }
    }

    let start_id = ids.reverse_lables["start"];
    let end_id = ids.reverse_lables["end"];
    let mut nodes_to_search = vec![Command::Push(start_id)];
    let mut path = vec![false; graph.len()];
    let mut duplicate_small_in_path = 0;
    let mut has_duplicate_small_in_path = false;
    let mut path_count = 0;

    while let Some(command) = nodes_to_search.pop() {
        match command {
            Command::Pop(poped_id) => {
                if poped_id == duplicate_small_in_path {
                    has_duplicate_small_in_path = false;
                    duplicate_small_in_path = -1;
                } else {
                    path[poped_id as usize] = false;
                }
            }
            Command::Push(node_id) => {
                let node = &graph[node_id as usize];
                let is_duplicate_small = !node.is_big && path[node_id as usize];
                if is_duplicate_small {
                    duplicate_small_in_path = node_id;
                    has_duplicate_small_in_path = true;
                }
                path[node_id as usize] = true;

                nodes_to_search.push(Command::Pop(node_id));

                if node_id == end_id {
                    path_count += 1;
                    continue;
                }

                for candidate in node.links.as_slice().iter().copied() {
                    if candidate == start_id {
                        continue;
                    }

                    let candidate_node = &graph[candidate as usize];

                    if !candidate_node.is_big
                        && has_duplicate_small_in_path
                        && path[candidate as usize]
                    {
                        continue;
                    }

                    nodes_to_search.push(Command::Push(candidate));
                }
            }
        }
    }

    path_count
}

fn main() {
    let input = get_input();
    let start = std::time::Instant::now();
    let res = solve(&input);
    let end = start.elapsed();
    println!(
        "Day {} ({:?}): {}",
        aoc2021::get_program_name(),
        end.as_micros(),
        res
    );
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        let input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

        assert_eq!(crate::solve(input), 36)
    }
}
