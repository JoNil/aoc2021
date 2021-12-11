use aoc2021::{get_input, parse_map};
fn solve(input: &str) -> i32 {
    let map = parse_map(input, |c| c.to_digit(10).unwrap() as i32);

    1656
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
