use aoc2021::get_input;

fn solve(input: &str) -> i32 {
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
        .fold((0, None), |(count, last), val| {
            if let Some(last) = last {
                if val > last {
                    (count + 1, Some(val))
                } else {
                    (count, Some(val))
                }
            } else {
                (count, Some(val))
            }
        })
        .0
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
