use std::collections::HashMap;

use aoc2021::get_input;
use glam::IVec2;
use parse_display::{Display, FromStr};

#[derive(FromStr, Display, PartialEq, Debug)]
#[display("{start.x},{start.y} -> {end.x},{end.y}")]
struct LineSegment {
    #[from_str(default)]
    start: IVec2,
    #[from_str(default)]
    end: IVec2,
}

impl LineSegment {
    fn draw(&self, map: &mut HashMap<IVec2, i32>) {
        let mut current = self.start;
        let mut dir = self.end - self.start;

        if dir.x != 0 {
            dir.x /= dir.x.abs();
        }

        if dir.y != 0 {
            dir.y /= dir.y.abs();
        }

        while current != self.end {
            *map.entry(current).or_default() += 1;
            current += dir;
        }

        *map.entry(current).or_default() += 1;
    }
}

fn solve(input: &str) -> i32 {
    let segments = input
        .lines()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<LineSegment>>();

    let mut map = HashMap::new();

    for segment in segments {
        segment.draw(&mut map);
    }

    map.values().copied().filter(|hits| *hits >= 2).count() as i32
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
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

        assert_eq!(crate::solve(input), 12)
    }
}
