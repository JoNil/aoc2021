use std::collections::HashSet;

use aoc2021::{get_input, parse_map};
use glam::{ivec2, IVec2};

fn solve(input: &str) -> i32 {
    let offsets = &[IVec2::new(0, -1), ivec2(-1, 0), ivec2(1, 0), ivec2(0, 1)];

    let map = parse_map(input, |c| c.to_digit(10).unwrap() as i32);

    let max_x = map
        .keys()
        .copied()
        .max_by(|p1, p2| p1.x.cmp(&p2.x))
        .unwrap();
    let max_y = map
        .keys()
        .copied()
        .max_by(|p1, p2| p1.y.cmp(&p2.y))
        .unwrap();

    let mut basin_start = Vec::new();

    for y in 0..=max_y.y {
        'next: for x in 0..=max_x.x {
            let pos = ivec2(x, y);
            let num = map.get(&pos).unwrap();

            for offset in offsets {
                let adjacent_num = map.get(&(pos + *offset)).unwrap_or(&i32::MAX);

                if *adjacent_num <= *num {
                    continue 'next;
                }
            }

            basin_start.push(ivec2(x, y));
        }
    }

    let mut basin_sizes = Vec::new();

    for start in basin_start {
        let mut positions_in_basing = HashSet::from([start]);
        let mut candidate_positions = vec![start];

        while !candidate_positions.is_empty() {
            let candidate = candidate_positions.pop().unwrap();

            for new_candidates in offsets.iter().map(|offset| candidate + *offset) {
                let height = map.get(&new_candidates).unwrap_or(&9);

                if *height >= 9 {
                    continue;
                }

                if !positions_in_basing.contains(&new_candidates) {
                    positions_in_basing.insert(new_candidates);
                    candidate_positions.push(new_candidates);
                }
            }
        }

        basin_sizes.push(positions_in_basing.len() as i32);
    }

    basin_sizes.sort_unstable();

    basin_sizes[(basin_sizes.len() - 3)..]
        .iter()
        .product::<i32>()
}

fn main() {
    let input = get_input();
    let res = solve(&input);
    println!("{}", res);
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        let input = "2199943210
3987894921
9856789892
8767896789
9899965678";

        assert_eq!(crate::solve(input), 1134)
    }
}
