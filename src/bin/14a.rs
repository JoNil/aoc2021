use aoc2021::get_input;
use std::collections::HashMap;

fn solve(input: &str) -> i32 {
    let mut parts = input.split("\n\n");

    let template_input = parts.next().unwrap();
    let rules_input = parts.next().unwrap();

    let mut template = template_input.as_bytes().to_vec();
    let mut rules = HashMap::<[u8; 2], u8>::new();

    for rule in rules_input.lines() {
        let mut rule_parts = rule.split(" -> ");
        let pattern = rule_parts.next().unwrap();
        let result = rule_parts.next().unwrap();

        rules.insert(
            pattern.as_bytes().try_into().unwrap(),
            *result.as_bytes().get(0).unwrap(),
        );
    }

    for _ in 0..10 {
        let mut insertions = Vec::new();

        for i in 0..(template.len() - 1) {
            let pair: [u8; 2] = (&template[i..(i + 2)]).try_into().unwrap();

            if let Some(insertion) = rules.get(&pair) {
                insertions.push((i + 1, *insertion));
            }
        }

        for (index, insertion) in insertions.iter().rev() {
            template.insert(*index, *insertion);
        }
    }

    let mut count = HashMap::<u8, i32>::new();

    for c in template {
        let count = count.entry(c).or_default();
        *count += 1;
    }

    count.values().max().unwrap() - count.values().min().unwrap()
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
        let input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
        assert_eq!(crate::solve(input), 1588)
    }
}
