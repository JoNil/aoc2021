use aoc2021::get_input;
use std::collections::HashMap;

fn solve(input: &str) -> i64 {
    let mut parts = input.split("\n\n");

    let template_input = parts.next().unwrap();
    let rules_input = parts.next().unwrap();

    let template = template_input.as_bytes();
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

    let mut pairs = HashMap::<[u8; 2], i64>::new();
    for i in 0..(template.len() - 1) {
        let pair: [u8; 2] = (&template[i..(i + 2)]).try_into().unwrap();
        let pair_count = pairs.entry(pair).or_default();
        *pair_count += 1;
    }

    for _ in 0..40 {
        let old_pairs = pairs.clone();

        for (pair, count) in old_pairs.iter() {
            if let Some(insertion) = rules.get(pair) {
                *pairs.get_mut(pair).unwrap() -= count;
                *pairs.entry([pair[0], *insertion]).or_default() += count;
                *pairs.entry([*insertion, pair[1]]).or_default() += count;
            }
        }
    }

    let mut counts = HashMap::<u8, i64>::new();
    *counts.entry(*template.last().unwrap()).or_default() += 1;

    for (pair, count) in pairs {
        *counts.entry(pair[0]).or_default() += count;
    }

    counts.values().max().unwrap() - counts.values().min().unwrap()
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
        assert_eq!(crate::solve(input), 2188189693529)
    }
}
