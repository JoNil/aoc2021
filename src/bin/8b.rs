use aoc2021::get_input;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl From<char> for Segment {
    fn from(c: char) -> Self {
        match c {
            'a' => Segment::A,
            'b' => Segment::B,
            'c' => Segment::C,
            'd' => Segment::D,
            'e' => Segment::E,
            'f' => Segment::F,
            'g' => Segment::G,
            _ => panic!("{} is not a Segment", c),
        }
    }
}

#[derive(Clone, Debug)]
struct Group {
    symbols: HashSet<Segment>,
}

impl FromStr for Group {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        let mut symbols = HashSet::new();

        for char in s.chars() {
            let segment: Segment = char.into();

            if symbols.contains(&segment) {
                panic!("We had duplicated segments :/")
            }

            symbols.insert(segment);
        }

        Ok(Self { symbols })
    }
}

fn solve(input: &str) -> i32 {
    let mut count = 0;

    for line in input.lines() {
        let mut input = Vec::new();
        let mut output = Vec::new();

        let mut five_groups = Vec::new();
        let mut six_groups = Vec::new();

        let mut digits = HashMap::new();

        let mut parts = line.split('|');

        for group in parts.next().unwrap().trim().split_whitespace() {
            let group_lenght = group.as_bytes().len();
            let group = group.parse::<Group>().unwrap();

            input.push(group.clone());

            match group_lenght {
                2 => {
                    digits.insert(1, group);
                }
                3 => {
                    digits.insert(7, group);
                }
                4 => {
                    digits.insert(4, group);
                }
                5 => {
                    five_groups.push(group);
                }
                6 => {
                    six_groups.push(group);
                }
                7 => {
                    digits.insert(8, group);
                }
                _ => (),
            }
        }

        for group in parts.next().unwrap().trim().split_whitespace() {
            let group = group.parse::<Group>().unwrap();
            output.push(group.clone());
        }

        {
            // Find the group for the digit 3
            for (i, candidate) in five_groups.iter().enumerate() {
                if candidate.symbols.difference(&digits[&7].symbols).count() == 2 {
                    digits.insert(3, candidate.clone());
                    five_groups.remove(i);
                    break;
                }
            }
        }

        {
            // Find the group for the digit 5 and 2
            for (i, candidate) in five_groups.iter().enumerate() {
                if candidate.symbols.difference(&digits[&4].symbols).count() == 2 {
                    digits.insert(5, candidate.clone());
                    five_groups.remove(i);
                    break;
                }
            }

            digits.insert(2, five_groups.drain(..).next().unwrap());
        }

        {
            // Find the group for the digit 9
            for (i, candidate) in six_groups.iter().enumerate() {
                if candidate.symbols.difference(&digits[&4].symbols).count() == 2 {
                    digits.insert(9, candidate.clone());
                    six_groups.remove(i);
                    break;
                }
            }
        }

        {
            // Find the group for the digit 0 and 6
            for (i, candidate) in six_groups.iter().enumerate() {
                if candidate.symbols.difference(&digits[&7].symbols).count() == 3 {
                    digits.insert(0, candidate.clone());
                    six_groups.remove(i);
                    break;
                }
            }

            digits.insert(6, six_groups.drain(..).next().unwrap());
        }

        let mut number = 0;

        for group in output.iter() {
            for (digit, candidate) in &digits {
                if group.symbols == candidate.symbols {
                    number *= 10;
                    number += digit;
                    break;
                }
            }
        }

        count += number;
    }
    count
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
        let input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

        assert_eq!(crate::solve(input), 61229)
    }
}
