use aoc2021::get_input;
use glam::IVec2;

fn parse_command(line: &str) -> IVec2 {
    match &line.split_ascii_whitespace().collect::<Vec<_>>()[..] {
        ["forward", amt] => IVec2::new(amt.parse::<i32>().unwrap(), 0),
        ["down", amt] => IVec2::new(0, amt.parse::<i32>().unwrap()),
        ["up", amt] => IVec2::new(0, -amt.parse::<i32>().unwrap()),
        _ => panic!("Invalid input"),
    }
}

fn solve(input: &str) -> i32 {
    let mut pos = IVec2::ZERO;

    for command in input.lines().map(parse_command) {
        pos += command;
    }

    pos.x * pos.y
}

fn main() {
    let input = get_input();
    let start = std::time::Instant::now();
    let res = solve(&input);
    let end = start.elapsed();
    println!("Day {} ({:?}): {}", aoc2021::get_program_name(), end, res);
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

        assert_eq!(crate::solve(input), 150)
    }
}
