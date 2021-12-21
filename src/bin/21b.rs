use aoc2021::get_input;

struct Die {
    state: i32,
    count: i64,
}

impl Die {
    fn toss(&mut self) -> i32 {
        let res = self.state;
        self.state += 1;
        self.count += 1;
        if self.state > 3 {
            self.state = 1;
        }
        res
    }
}

fn solve(input: &str) -> i64 {
    let mut lines = input.lines();
    let mut player_1_pos = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();
    let mut player_2_pos = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let mut player_1_score = 0;
    let mut player_2_score = 0;

    let mut die = Die { state: 1, count: 0 };

    loop {
        {
            let steps = die.toss() + die.toss() + die.toss();

            player_1_pos = ((player_1_pos - 1) + steps) % 10 + 1;
            player_1_score += player_1_pos as i64;

            if player_1_score >= 21 {
                return player_2_score * die.count;
            }
        }

        {
            let steps = die.toss() + die.toss() + die.toss();

            player_2_pos = ((player_2_pos - 1) + steps) % 10 + 1;
            player_2_score += player_2_pos as i64;

            if player_2_score >= 21 {
                return player_1_score * die.count;
            }
        }
    }
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
        let input = "Player 1 starting position: 4
    Player 2 starting position: 8";
        assert_eq!(crate::solve(input), 444356092776315);
    }
}
