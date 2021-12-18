use aoc2021::get_input;
use std::fmt::Debug;
use Number::{Literal, Pair};

enum Number {
    Literal(i32),
    Pair(Box<Number>, Box<Number>),
}

impl Number {
    fn is_regular_pair(&self) -> Option<(i32, i32)> {
        match self {
            Pair(left, right) => match (&**left, &**right) {
                (Literal(lhs), Literal(rhs)) => Some((*lhs, *rhs)),
                _ => None,
            },
            _ => None,
        }
    }

    fn explode(self, depth: i32) -> (Option<(i32, i32)>, Number) {
        if depth >= 4 {
            if let Some((lhs, rhs)) = self.is_regular_pair() {
                return (Some((lhs, rhs)), Literal(0));
            }
        }

        match self {
            Literal(val) => (None, Literal(val)),
            Pair(lhs, rhs) => {
                let lhs = match lhs.explode(depth + 1) {
                    (Some((left, right)), num) => {
                        return (Some((left, right)), Pair(Box::new(num), rhs));
                    }
                    (None, lhs) => lhs,
                };

                let rhs = match rhs.explode(depth + 1) {
                    (Some((left, right)), num) => {
                        return (Some((left, right)), Pair(Box::new(lhs), Box::new(num)));
                    }
                    (None, rhs) => rhs,
                };

                (None, Pair(Box::new(lhs), Box::new(rhs)))
            }
        }
    }

    fn reduce(self) -> Number {
        self.explode(1).1
    }

    fn add(self, rhs: Number) -> Number {
        Pair(Box::new(self), Box::new(rhs)).reduce()
    }
}

impl Debug for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal(value) => write!(f, "{:?}", value),
            Pair(left, right) => write!(f, "[{:?},{:?}]", left, right),
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
            let res = Number::Pair(Box::new(left), Box::new(right));

            assert!(self.take_byte()? == b']');

            Some(res)
        } else {
            Some(Number::Literal((byte as char).to_digit(10).unwrap() as i32))
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

    #[test]
    fn test_explode() {
        use crate::Parser;

        let data = [
            ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
            ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
            ("[[[[[9,8],1],2],3],4]", "[[6,[5,[4,[3,2]]]],1]"),
        ];

        for (input, output) in data {
            let mut number = Parser::new(input).take_number().unwrap().reduce();
            assert_eq!(format!("{:?}", number), output);
        }
    }
}
