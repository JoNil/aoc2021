use aoc2021::get_input;

fn calc_score(c: char) -> i64 {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!("Invalid char"),
    }
}

fn get_matchin(c: char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => panic!("Invalid char"),
    }
}

fn solve(input: &str) -> i64 {
    let mut line_scores = Vec::new();

    'next_line: for line in input.lines() {
        let mut stack = Vec::new();

        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' | ']' | '}' | '>' => {
                    let last = stack.pop().unwrap();

                    if last != get_matchin(c) {
                        continue 'next_line;
                    }
                }
                _ => panic!("Invalid input"),
            }
        }

        let mut line_score = 0;

        for compleating_c in stack.iter().rev() {
            line_score = line_score * 5 + calc_score(*compleating_c);
        }

        line_scores.push(line_score);
    }

    line_scores.sort_unstable();

    line_scores[line_scores.len() / 2]
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
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

        assert_eq!(crate::solve(input), 288957)
    }
}
