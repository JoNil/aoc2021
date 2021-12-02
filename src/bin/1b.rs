use aoc2021::get_input;

fn solve(input: &str) -> i32 {
    let mut last = None;

    let input = input
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut window_sums = Vec::new();

    for window_start in 0..(input.len() - 2) {
        window_sums.push(input[window_start..(window_start + 3)].iter().sum::<i32>());
    }

    window_sums
        .iter()
        .map(|v| {
            let res = if let Some(last) = last {
                if v > last {
                    1
                } else {
                    0
                }
            } else {
                0
            };

            last = Some(v);

            res
        })
        .sum::<i32>()
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
        let input = "199
200
208
210
200
207
240
269
260
263";

        assert_eq!(crate::solve(input), 5)
    }
}
