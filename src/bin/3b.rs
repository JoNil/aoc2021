use core::num;

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

fn solve(input: &str) -> u32 {
    let mut bit_count = 0;

    let mut numbers = Vec::new();

    for line in input.lines() {
        bit_count = line.len();
        numbers.push(u32::from_str_radix(line, 2).unwrap());
    }

    let mut oxygen_generator_rating = 0;
    let mut remaining_oxygen = numbers.clone();

    for bit in 0..bit_count {
        let (one_count, zero_count) = count_bit_digits(bit_count, remaining_oxygen.as_slice());

        let desired_value = if dbg!(one_count[bit]) >= dbg!(zero_count[bit]) {
            1
        } else {
            0
        };

        let bit_pos = bit_count - 1 - bit;

        remaining_oxygen = remaining_oxygen
            .iter()
            .copied()
            .filter(|num| ((*num & (1 << bit_pos)) >> bit_pos) == desired_value)
            .collect::<Vec<_>>();

        if remaining_oxygen.len() == 1 {
            oxygen_generator_rating = *remaining_oxygen.first().unwrap();
            break;
        }
    }

    let mut co2_scrubber_rating = 0;
    let mut remaining_co2_scrubber = numbers.clone();

    for bit in 0..bit_count {
        let (one_count, zero_count) =
            count_bit_digits(bit_count, remaining_co2_scrubber.as_slice());

        let desired_value = if dbg!(one_count[bit]) < dbg!(zero_count[bit]) {
            1
        } else {
            0
        };

        let bit_pos = bit_count - 1 - bit;

        remaining_co2_scrubber = remaining_co2_scrubber
            .iter()
            .copied()
            .filter(|num| ((*num & (1 << bit_pos)) >> bit_pos) == desired_value)
            .collect::<Vec<_>>();

        if remaining_co2_scrubber.len() == 1 {
            co2_scrubber_rating = *remaining_co2_scrubber.first().unwrap();
            break;
        }
    }

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
