use aoc2021::get_input;
use std::fmt::{Debug, Display, Write};

#[derive(Copy, Clone, Debug)]
struct Num {
    depth: i32,
    value: i32,
}

type Number = Vec<Num>;

fn print(num: &Number) -> String {
    let mut res = "".to_string();

    dbg!(num);

    let mut last_level = 0;
    let mut current_level = 0;
    let mut printed_at_level = 0;
    let mut needs_comma = false;
    for (i, n) in num.iter().enumerate() {
        while current_level < n.depth {
            if needs_comma {
                write!(res, ",").unwrap();
                needs_comma = false;
            }

            write!(res, "[").unwrap();
            current_level += 1;
            printed_at_level = 1;
        }

        while current_level > n.depth {
            write!(res, "]").unwrap();
            current_level -= 1;
            printed_at_level = 0;
        }

        if printed_at_level == 2 {
            write!(res, "],[").unwrap();
            printed_at_level = 0;
            needs_comma = false;
        }

        if needs_comma {
            write!(res, ",").unwrap();
            needs_comma = false;
        }

        write!(res, "{}", n.value).unwrap();
        needs_comma = true;

        if current_level == last_level {
            printed_at_level += 1;
        }

        last_level = current_level;
    }

    while current_level > 0 {
        write!(res, "]").unwrap();
        current_level -= 1;
    }

    res
}

fn parse(input: &str) -> Number {
    let mut res = Vec::new();
    let mut current_depth = 0;

    for c in input.chars() {
        match c {
            '[' => current_depth += 1,
            ']' => current_depth -= 1,
            n @ '0'..='9' => res.push(Num {
                depth: current_depth,
                value: n.to_digit(10).unwrap() as i32,
            }),
            _ => continue,
        }
    }

    res
}

fn add(mut a: Number, b: Number) -> Number {
    a.extend_from_slice(&b);
    for num in a.iter_mut() {
        num.depth += 1;
    }
    a
}

fn reduce(mut num: Number) -> Number {
    for i in 0..(num.len() - 1) {
        let a = num[i];
        let b = num[i + 1];

        if a.depth >= 4 && b.depth >= 4 && a.depth == b.depth {
            if i > 0 {
                num[i - 1].value += a.value;
            }

            num[i].value = 0;
            num[i].depth -= 1;
            num.remove(i + 1);

            if i + 1 < num.len() {
                num[i + 1].value += b.value
            }
            break;
        }
    }

    num
}

fn solve(input: &str) -> i32 {
    let mut lines = input.lines();

    let mut sum = parse(lines.next().unwrap());

    for line in lines {
        let new_num = parse(line);
        sum = add(sum, new_num);
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
        let data = [
            ("[[9,8],[1,2],[1,2]]", "[[9,8],[1,2],[1,2]]"),
            ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
            ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
            ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
            (
                "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            ),
            (
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
            ),
        ];

        for (input, output) in data {
            assert_eq!(crate::print(&crate::reduce(crate::parse(input))), output);
        }
    }
}
