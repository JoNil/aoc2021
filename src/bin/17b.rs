use aoc2021::get_input;
use glam::{ivec2, IVec2};
use parse_display::{Display, FromStr};

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

    let mut hits = 0;

    for dx in 0..2 * target.end.x {
        for dy in -1000..2000 {
            let mut pos = ivec2(0, 0);
            let mut velocity = ivec2(dx, dy);

            for _ in 0.. {
                pos += velocity;
                velocity.x = (velocity.x - 1).max(0);
                velocity.y -= 1;

                // Collided
                if pos.x <= target.end.x
                    && pos.x >= target.start.x
                    && pos.y >= target.start.y
                    && pos.y <= target.end.y
                {
                    hits += 1;
                    break;
                }

                // Stop if we are past
                if pos.x > target.end.x || pos.y < target.start.y {
                    break;
                }
            }
        }
    }

    hits
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
        assert_eq!(crate::solve(input), 112);
    }
}
