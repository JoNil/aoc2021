use aoc2021::get_input;

fn solve(input: &str) -> i64 {
    let fish_list = input
        .trim()
        .split(',')
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut fish_at_age = vec![0; 9];

    for fish in fish_list {
        fish_at_age[fish as usize] += 1;
    }

    for _ in 0..256 {
        fish_at_age.rotate_left(1);
        fish_at_age[6] += fish_at_age[8];
    }

    fish_at_age.iter().sum()
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
        let input = "3,4,3,1,2";

        assert_eq!(crate::solve(input), 26984457539)
    }
}
