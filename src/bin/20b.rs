use aoc2021::{get_input, parse_map};
use glam::{const_ivec2, ivec2, IVec2};
use std::collections::HashMap;

static OFFSETS: &[IVec2] = &[
    const_ivec2!([-1, -1]),
    const_ivec2!([0, -1]),
    const_ivec2!([1, -1]),
    const_ivec2!([-1, 0]),
    const_ivec2!([0, 0]),
    const_ivec2!([1, 0]),
    const_ivec2!([-1, 1]),
    const_ivec2!([0, 1]),
    const_ivec2!([1, 1]),
];

fn solve(input: &str) -> i32 {
    let mut parts = input.split("\n\n");

    let algo = parts.next().unwrap().chars().collect::<Vec<_>>();

    let mut map = parse_map(parts.next().unwrap(), |c| c);

    let mut outside = '.';

    for _ in 0..50 {
        let min_x = map.keys().map(|v| v.x).min().unwrap();
        let min_y = map.keys().map(|v| v.y).min().unwrap();

        let max_x = map.keys().map(|v| v.x).max().unwrap();
        let max_y = map.keys().map(|v| v.y).max().unwrap();

        let mut new_map = HashMap::new();

        for y in (min_y - 1)..=(max_y + 1) {
            for x in (min_x - 1)..=(max_x + 1) {
                let mut index = 0;

                let pos = ivec2(x, y);

                for offset in OFFSETS {
                    let input = pos + *offset;

                    let old = map.get(&input).unwrap_or(&outside);

                    index = (index << 1) | if *old == '.' { 0 } else { 1 };
                }

                new_map.insert(pos, algo[index as usize]);
            }
        }

        outside = if outside == '.' {
            *algo.first().unwrap()
        } else {
            *algo.last().unwrap()
        };

        map = new_map;
    }

    map.values().copied().filter(|c| *c == '#').count() as i32
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
        let input = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";
        assert_eq!(crate::solve(input), 3351);
    }
}
