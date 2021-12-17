use aoc2021::get_input;
use glam::{ivec2, IVec2};
use parse_display::{Display, FromStr};
use std::{collections::HashMap, fmt};

#[derive(FromStr, Display, PartialEq, Debug)]
#[display("target area: x={start.x}..{end.x}, y={start.y}..{end.y}")]
struct TargetArea {
    #[from_str(default)]
    start: IVec2,
    #[from_str(default)]
    end: IVec2,
}

fn solve(input: &str) -> i32 {
    let target = input.trim().parse::<TargetArea>().unwrap();

    let mut max_y = 0;

    for dx in 1..target.end.x {
        for dy in target.start.y..1000 {
            let mut pos = ivec2(0, 0);
            let mut velocity = ivec2(dx, dy);
            let mut local_max_y = 0;

            for _ in 0.. {
                pos += velocity;
                velocity.x = (velocity.x - 1).max(0);
                velocity.y -= 1;

                if pos.y > local_max_y {
                    local_max_y = pos.y;
                }

                // Collided
                if pos.x <= target.end.x
                    && pos.x >= target.start.x
                    && pos.y >= target.start.y
                    && pos.y <= target.end.y
                {
                    if local_max_y > max_y {
                        max_y = local_max_y;
                    }
                    break;
                }

                // Stop if we are past
                if pos.x > target.end.x || pos.y < target.start.y {
                    break;
                }
            }
        }
    }

    max_y
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
        let input = "target area: x=20..30, y=-10..-5";
        assert_eq!(crate::solve(input), 45);
    }
}
