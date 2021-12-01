use aoc2021::get_input;

fn solve(input: impl AsRef<str>) -> i32 {
    let mut last = None;

    input
        .as_ref()
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
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
    let res = solve(input);
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

        assert_eq!(crate::solve(input), 7)
    }
}
