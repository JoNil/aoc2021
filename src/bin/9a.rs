use std::collections::HashMap;

use aoc2021::get_input;
use glam::{ivec2, IVec2};

fn solve(input: &str) -> i32 {
    let mut map = HashMap::new();

    let offsets = &[IVec2::new(0, -1), ivec2(-1, 0), ivec2(1, 0), ivec2(0, 1)];

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            let pos = ivec2(x as i32, y as i32);
            let height = char.to_digit(10).unwrap() as i32;
            map.insert(pos, height);
        }
    }

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

    let mut risk = 0;

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

            risk += 1 + num;
        }
    }

    risk
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

        assert_eq!(crate::solve(input), 15)
    }
}
