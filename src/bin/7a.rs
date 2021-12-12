use aoc2021::get_input;

fn sum_of_distances(nums: &[i32], target: i32) -> i32 {
    nums.iter().fold(0, |sum, val| sum + (*val - target).abs())
}

fn solve(input: &str) -> i32 {
    let numbers = input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i32>>();

    let sum = numbers.iter().sum::<i32>();

    let average = sum as f32 / numbers.len() as f32;

    let mut has_switched_direction = false;
    let mut current_guess = average as i32;
    let mut lowest_so_far = sum_of_distances(&numbers, current_guess);
    let mut direction = 1;

    loop {
        let new_guess = current_guess + direction;
        let new_value = sum_of_distances(&numbers, new_guess);

        if new_value < lowest_so_far {
            current_guess = new_guess;
            lowest_so_far = new_value;
        } else if !has_switched_direction {
            direction = -1;
            has_switched_direction = true;
        } else {
            break;
        }
    }

    lowest_so_far
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
        let input = "16,1,2,0,4,2,7,1,2,14";

        assert_eq!(crate::solve(input), 37)
    }
}
