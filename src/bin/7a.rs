use aoc2021::get_input;

fn sum_of_distances(nums: &[i32], target: i32) -> i32 {
    nums.iter().fold(0, |sum, val| sum + (*val - target).abs())
}

fn solve(input: &str) -> i64 {
    let numbers = input
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i32>>();

    let sum = numbers.iter().sum::<i32>();

    let average = sum as f32 / numbers.len() as f32;

    println!("{}", sum_of_distances(&numbers, 2));

    println!("{}", average);

    0
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
        let input = "16,1,2,0,4,2,7,1,2,14";

        assert_eq!(crate::solve(input), 37)
    }
}
