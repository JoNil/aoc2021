use aoc2021::{get_input, parse_map};
use glam::ivec2;
use std::collections::HashSet;

fn solve(input: &str) -> i32 {
    let offsets = &[
        ivec2(-1, -1),
        ivec2(0, -1),
        ivec2(1, -1),
        ivec2(-1, 0),
        ivec2(1, 0),
        ivec2(-1, 1),
        ivec2(0, 1),
        ivec2(1, 1),
    ];

    let mut map = parse_map(input, |c| c.to_digit(10).unwrap() as i32);

    let positions = map.keys().copied().collect::<Vec<_>>();

    let mut flash_count = 0;

    for _ in 0..100 {
        let mut flashes = HashSet::new();
        let mut about_to_flash = Vec::new();

        for value in map.values_mut() {
            *value += 1;
        }

        for pos in &positions {
            let value = map.get(pos).unwrap();

            if *value > 9 {
                about_to_flash.push(*pos);
            }
        }

        while let Some(pos) = about_to_flash.pop() {
            if flashes.contains(&pos) {
                continue;
            }

            flashes.insert(pos);

            for offset in offsets {
                let pos = pos + *offset;

                if map.contains_key(&pos) {
                    let value = {
                        let value = map.get_mut(&pos).unwrap();
                        *value += 1;
                        *value
                    };

                    if value > 9 {
                        about_to_flash.push(pos);
                    }
                }
            }
        }

        for flash in &flashes {
            *map.get_mut(flash).unwrap() = 0;
        }

        flash_count += flashes.len();
    }

    flash_count as i32
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
        let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

        assert_eq!(crate::solve(input), 1656)
    }
}
