use aoc2021::get_input;

fn count_bit_digits(bit_count: usize, numbers: &[u32]) -> (Vec<u32>, Vec<u32>) {
    let mut one_count = vec![0; bit_count];
    let mut zero_count = vec![0; bit_count];

    for num in numbers {
        for bit in 0..bit_count {
            if num & (1 << (bit_count - 1 - bit)) > 0 {
                one_count[bit] += 1;
            } else {
                zero_count[bit] += 1;
            }
        }
    }

    (one_count, zero_count)
}

fn search_for_pattern(
    bit_count: usize,
    numbers: &[u32],
    selector: impl Fn(u32, u32) -> bool,
) -> u32 {
    let mut remaining_numbers = numbers.to_vec();

    for bit in 0..bit_count {
        let (one_count, zero_count) = count_bit_digits(bit_count, remaining_numbers.as_slice());

        let desired_value = if selector(one_count[bit], zero_count[bit]) {
            1
        } else {
            0
        };

        let bit_pos = bit_count - 1 - bit;

        remaining_numbers = remaining_numbers
            .iter()
            .copied()
            .filter(|num| ((*num & (1 << bit_pos)) >> bit_pos) == desired_value)
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

    let oxygen_generator_rating =
        search_for_pattern(bit_count, &numbers, |one_count, zero_count| {
            one_count >= zero_count
        });
    let co2_scrubber_rating = search_for_pattern(bit_count, &numbers, |one_count, zero_count| {
        one_count < zero_count
    });

    oxygen_generator_rating * co2_scrubber_rating
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
