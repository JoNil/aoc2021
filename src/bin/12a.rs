use aoc2021::get_input;
use std::collections::HashMap;

#[derive(Debug, Default)]
struct Node {
    links: Vec<i32>,
    is_big: bool,
}

#[derive(Default)]
struct IdTable<'a> {
    lables: Vec<&'a str>,
    reverse_lables: HashMap<&'a str, i32>,
    next_id: i32,
}

impl<'a> IdTable<'a> {
    fn add(&mut self, s: &'a str) -> i32 {
        *self.reverse_lables.entry(s).or_insert_with(|| {
            let id = self.next_id;
            self.next_id += 1;
            self.lables.push(s);
            id
        })
    }
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
            node.is_big = ids.lables[from_id as usize]
                .chars()
                .next()
                .unwrap()
                .is_uppercase();
        }
        {
            let node = &mut graph[to_id as usize];
            node.links.push(from_id);
            node.is_big = ids.lables[to_id as usize]
                .chars()
                .next()
                .unwrap()
                .is_uppercase();
        }
    }

    let end_id = ids.reverse_lables["end"];
    let mut nodes_to_search = vec![ids.reverse_lables["start"]];
    let mut path = Vec::new();
    let mut path_count = 0;

    while let Some(node_id) = nodes_to_search.pop() {
        if node_id == -1 {
            path.pop();
            continue;
        }

        let node = &graph[node_id as usize];
        path.push(node_id);

        nodes_to_search.push(-1);

        if node_id == end_id {
            path_count += 1;
            continue;
        }

        for candidate in node.links.as_slice().iter().copied() {
            let candidate_node = &graph[candidate as usize];

            if !candidate_node.is_big && path.contains(&candidate) {
                continue;
            }

            nodes_to_search.push(candidate);
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

        assert_eq!(crate::solve(input), 10)
    }
}
