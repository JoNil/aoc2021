use std::fmt::Debug;

use aoc2021::get_input;

enum Number {
    Literal {
        value: i32,
    },
    Pair {
        left: Box<Number>,
        right: Box<Number>,
    },
}

impl Number {
    fn add(self, rhs: Number) -> Number {
        Number::Pair {
            left: Box::new(self),
            right: Box::new(rhs),
        }
    }
}

impl Debug for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Literal { value } => write!(f, "{:?}", value),
            Number::Pair { left, right } => write!(f, "[{:?},{:?}]", left, right),
        }
    }
}

struct Parser<'a> {
    index: usize,
    data: &'a [u8],
}

impl<'a> Parser<'a> {
    fn new(data: &'a str) -> Self {
        Self {
            index: 0,
            data: data.as_bytes(),
        }
    }

    fn take_byte(&mut self) -> Option<u8> {
        let res = self.data.get(self.index).copied();
        self.index += 1;
        res
    }

    fn take_number(&mut self) -> Option<Number> {
        let mut byte = self.take_byte()?;

        while byte.is_ascii_whitespace() {
            byte = self.take_byte()?;
        }

        if byte == b'[' {
            let left = self.take_number()?;

            assert!(self.take_byte()? == b',');

            let right = self.take_number()?;
            let res = Number::Pair {
                left: Box::new(left),
                right: Box::new(right),
            };

            assert!(self.take_byte()? == b']');

            Some(res)
        } else {
            Some(Number::Literal {
                value: (byte as char).to_digit(10).unwrap() as i32,
            })
        }
    }
}

fn solve(input: &str) -> i32 {
    let mut parser = Parser::new(input.trim());

    let mut sum = parser.take_number().unwrap();

    while let Some(number) = parser.take_number() {
        sum = sum.add(number);
    }

    dbg!(sum);

    0
}

fn main() {
    let input = get_input();
    let input = "[1,2]
[[3,4],5]"
        .to_string();
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
        let input = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
        assert_eq!(crate::solve(input), 4140);
    }
}
