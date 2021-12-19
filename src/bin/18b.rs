use aoc2021::get_input;
use itertools::Itertools;
use std::fmt::Debug;

#[derive(Copy, Clone, Debug)]
struct Num {
    depth: i32,
    value: i32,
}

type Number = Vec<Num>;

fn magnitude(mut num: Number) -> i32 {
    let mut current_depth = num.iter().map(|n| n.depth).max().unwrap();

    while num.len() > 1 {
        let mut i = 0;
        while i < num.len() - 1 {
            if num[i].depth == current_depth && num[i + 1].depth == current_depth {
                num[i].depth -= 1;
                num[i].value = 3 * num[i].value + 2 * num[i + 1].value;
                num.remove(i + 1);
            }
            i += 1;
        }
        current_depth -= 1;
    }

    num.into_iter().next().unwrap().value
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

fn explode(mut num: Number) -> (bool, Number) {
    for i in 0..(num.len() - 1) {
        let a = num[i];
        let b = num[i + 1];

        if a.depth > 4 && b.depth > 4 && a.depth == b.depth {
            if i > 0 {
                num[i - 1].value += a.value;
            }

            num[i].value = 0;
            num[i].depth -= 1;
            num.remove(i + 1);

            if i + 1 < num.len() {
                num[i + 1].value += b.value
            }
            return (true, num);
        }
    }

    (false, num)
}

fn split(mut num: Number) -> (bool, Number) {
    for i in 0..num.len() {
        if num[i].value > 9 {
            num[i].depth += 1;

            let left = num[i].value / 2;
            let right = (num[i].value + 1) / 2;

            num[i].value = left;

            num.insert(
                i + 1,
                Num {
                    depth: num[i].depth,
                    value: right,
                },
            );

            return (true, num);
        }
    }

    (false, num)
}

fn reduce(mut num: Number) -> Number {
    loop {
        let (applied, res) = explode(num);
        num = res;

        if applied {
            continue;
        }

        let (applied, res) = split(num);
        num = res;

        if applied {
            continue;
        }

        break;
    }

    num
}

fn solve(input: &str) -> i32 {
    let numbers = input.lines().map(parse).collect::<Vec<_>>();

    let mut max = 0;

    let mut permutations = Vec::new();
    for (a, b) in (0..numbers.len()).tuple_combinations() {
        permutations.push((a, b));
        permutations.push((b, a));
    }

    for (a, b) in permutations {
        max = max.max(magnitude(reduce(add(
            numbers[a].clone(),
            numbers[b].clone(),
        ))))
    }

    max
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
        assert_eq!(crate::solve(input), 3993);
    }
}
