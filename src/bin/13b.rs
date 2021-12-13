use std::{collections::HashSet, mem};

use aoc2021::get_input;

#[derive(Debug)]
enum Fold {
    Vertical(i32),
    Horizontal(i32),
}

fn print_map(map: &HashSet<(i32, i32)>) {
    let max_x = map
        .iter()
        .copied()
        .max_by(|p1, p2| p1.0.cmp(&p2.0))
        .unwrap();
    let max_y = map
        .iter()
        .copied()
        .max_by(|p1, p2| p1.1.cmp(&p2.1))
        .unwrap();

    for y in 0..=max_y.1 {
        for x in 0..=max_x.0 {
            if map.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn solve(input: &str) -> i32 {
    let mut map = HashSet::new();

    let mut input = input.split("\n\n");

    let points = input.next().unwrap();

    for point in points.trim().lines() {
        let mut split = point.split(',');

        let x = split.next().unwrap().parse::<i32>().unwrap();
        let y = split.next().unwrap().parse::<i32>().unwrap();

        map.insert((x, y));
    }

    let mut folds = Vec::new();

    for fold in input.next().unwrap().lines() {
        let mut fold_split = fold.split('=');

        let fold_axis = *fold_split.next().unwrap().as_bytes().last().unwrap();
        let fold_position = fold_split.next().unwrap().parse::<i32>().unwrap();

        folds.push(match fold_axis {
            b'x' => Fold::Vertical(fold_position),
            b'y' => Fold::Horizontal(fold_position),
            _ => panic!("Invalid fold axis"),
        });
    }

    let mut new_map = HashSet::new();

    for fold in &folds {
        for (x, y) in map.iter().copied() {
            new_map.insert(match fold {
                Fold::Vertical(pos) => {
                    if x > *pos {
                        (2 * *pos - x, y)
                    } else {
                        (x, y)
                    }
                }
                Fold::Horizontal(pos) => {
                    if y > *pos {
                        (x, 2 * *pos - y)
                    } else {
                        (x, y)
                    }
                }
            });
        }

        mem::swap(&mut map, &mut new_map);
        new_map.clear();
    }

    print_map(&map);

    0
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
        let input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
        assert_eq!(crate::solve(input), 17)
    }
}
