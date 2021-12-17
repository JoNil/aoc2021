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

pub fn print_map(map: &HashMap<IVec2, impl fmt::Display>) {
    let min_x = map.keys().map(|v| v.x).min().unwrap();
    let min_y = map.keys().map(|v| v.y).min().unwrap();

    let max_x = map.keys().map(|v| v.x).max().unwrap();
    let max_y = map.keys().map(|v| v.y).max().unwrap();

    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            let hit = map
                .get(&ivec2(x, y))
                .map(|count| format!("{}", count))
                .unwrap_or_else(|| ".".to_string());

            print!("{}", hit);
        }
        println!();
    }
}

fn solve(input: &str) -> i32 {
    let target = input.trim().parse::<TargetArea>().unwrap();

    let mut map = HashMap::new();

    for y in target.start.y..=target.end.y {
        for x in target.start.x..=target.end.x {
            map.insert(ivec2(x, y), 'T');
        }
    }

    let mut max_y = 0;

    let mut pos = ivec2(0, 0);
    let mut velocity = ivec2(6, 4);

    for _ in 0.. {
        pos += velocity;
        velocity.x = (velocity.x - 1).max(0);
        velocity.y -= 1;

        if pos.y > max_y {
            max_y = pos.y;
        }

        // Collided
        if pos.x <= target.end.x
            && pos.x >= target.start.x
            && pos.y >= target.start.y
            && pos.y <= target.end.y
        {
            map.insert(pos, 'B');
            println!("Collided!");
            break;
        }

        // Stop if we are past
        if pos.x > target.end.x || pos.y < target.start.y {
            dbg!(pos);
            dbg!(target.end);
            break;
        }

        map.insert(pos, '#');
    }

    map.insert(ivec2(0, 0), 'S');

    dbg!(max_y);

    print_map(&map);

    //dbg!(&map);

    0
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
