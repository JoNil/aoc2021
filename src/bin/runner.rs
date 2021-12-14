use parse_display::{Display, FromStr};
use std::{process::Command, str};

#[derive(FromStr, Display, PartialEq, Debug)]
#[display("Day {program} ({time}): {result}")]
struct Result {
    program: String,
    time: String,
    result: String,
}

fn main() {
    let mut total = 0;

    for i in 1..=14 {
        let a = Command::new(format!("target/release/{}a", i))
            .output()
            .unwrap();

        let a_res = str::from_utf8(&a.stdout)
            .unwrap()
            .trim()
            .lines()
            .last()
            .unwrap()
            .parse::<Result>()
            .unwrap();

        total += a_res.time.parse::<i32>().unwrap();

        println!(
            "Day {:<3} {:<12} {}",
            a_res.program,
            format!("({} μs):", a_res.time),
            a_res.result
        );

        let b = Command::new(format!("target/release/{}b", i))
            .output()
            .unwrap();

        let b_res = str::from_utf8(&b.stdout)
            .unwrap()
            .trim()
            .lines()
            .last()
            .unwrap()
            .parse::<Result>()
            .unwrap();

        total += b_res.time.parse::<i32>().unwrap();

        println!(
            "Day {:<3} {:<12} {}",
            b_res.program,
            format!("({} μs):", b_res.time),
            b_res.result
        );
    }

    println!("Total {} μs", total);
}
