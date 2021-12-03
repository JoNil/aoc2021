use aoc2021::get_input;

fn solve(input: &str) -> u32 {
    let mut bit_count = 0;

    let mut one_count = Vec::new();
    let mut zero_count = Vec::new();

    for line in input.lines() {
        bit_count = line.len();

        if one_count.is_empty() {
            one_count.resize(line.len(), 0);
        }

        if zero_count.is_empty() {
            zero_count.resize(line.len(), 0);
        }

        let num = u32::from_str_radix(line, 2).unwrap();

        for bit in 0..bit_count {
            if num & (1 << (bit_count - 1 - bit)) > 0 {
                one_count[bit] += 1;
            } else {
                zero_count[bit] += 1;
            }
        }
    }

    let mut gamma_rate = 0;

    for bit in 0..bit_count {
        if one_count[bit] > zero_count[bit] {
            gamma_rate = (gamma_rate << 1) | 0b1;
        } else {
            gamma_rate <<= 1;
        }
    }

    let epsilon_rate = !gamma_rate & (!-(1 << bit_count as i32)) as u32;

    gamma_rate * epsilon_rate
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

        assert_eq!(crate::solve(input), 198)
    }
}
