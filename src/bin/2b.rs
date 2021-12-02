use aoc2021::get_input;
use glam::IVec2;

enum Command {
    Aim(i32),
    Fwd(i32),
}

fn parse_command(line: &str) -> Command {
    match &line.split_ascii_whitespace().collect::<Vec<_>>()[..] {
        ["forward", amt] => Command::Fwd(amt.parse::<i32>().unwrap()),
        ["down", amt] => Command::Aim(amt.parse::<i32>().unwrap()),
        ["up", amt] => Command::Aim(-amt.parse::<i32>().unwrap()),
        _ => panic!("Invalid input"),
    }
}

fn solve(input: &str) -> i32 {
    let mut pos = IVec2::ZERO;
    let mut aim = 0;

    for command in input.lines().map(parse_command) {
        match command {
            Command::Aim(amt) => aim += amt,
            Command::Fwd(amt) => {
                pos += IVec2::new(amt, aim * amt);
            }
        }
    }

    pos.x * pos.y
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
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

        assert_eq!(crate::solve(input), 900)
    }
}
