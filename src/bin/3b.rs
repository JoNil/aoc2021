use aoc2021::get_input;
use bit_field::BitField;

fn count_bit_digits(bit_count: usize, numbers: &[u32]) -> Vec<i32> {
    let mut count = vec![0; bit_count];

    for num in numbers {
        for (i, c) in count.iter_mut().enumerate() {
            let bit = num.get_bit(bit_count - 1 - i);
            *c += if bit { 1 } else { -1 };
        }
    }

    count
}

fn search_for_pattern(bit_count: usize, numbers: &[u32], selector: impl Fn(i32) -> bool) -> u32 {
    let mut remaining_numbers = numbers.to_vec();

    for bit in 0..bit_count {
        let count = count_bit_digits(bit_count, remaining_numbers.as_slice());

        remaining_numbers = remaining_numbers
            .iter()
            .copied()
            .filter(|num| num.get_bit(bit_count - 1 - bit) == selector(count[bit]))
            .collect::<Vec<_>>();

        if remaining_numbers.len() == 1 {
            return *remaining_numbers.first().unwrap();
        }
    }

    panic!("Didn't find value");
}

fn solve(input: &str) -> u32 {
    let mut bit_count = 0;

    let mut numbers = Vec::new();

    for line in input.lines() {
        bit_count = line.len();
        numbers.push(u32::from_str_radix(line, 2).unwrap());
    }

    let oxygen_generator_rating = search_for_pattern(bit_count, &numbers, |count| count >= 0);
    let co2_scrubber_rating = search_for_pattern(bit_count, &numbers, |count| count < 0);

    oxygen_generator_rating * co2_scrubber_rating
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
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

        assert_eq!(crate::solve(input), 230)
    }
}
