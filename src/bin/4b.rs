use std::{collections::HashMap, fmt::Debug};

use aoc2021::get_input;

#[derive(Default, Debug)]
struct Board {
    hit_counter_x: [i32; 5],
    hit_counter_y: [i32; 5],
    data: HashMap<i32, (usize, usize)>,
    delete: bool,
}

fn parse_board(s: &str) -> Board {
    let mut res = Board::default();

    for (y, line) in s.lines().enumerate() {
        for (x, val) in line.trim().split_whitespace().enumerate() {
            res.data.insert(val.parse().unwrap(), (x, y));
        }
    }

    res
}

fn solve(input: &str) -> i32 {
    let mut segments = input.split("\n\n");

    let numbers = segments
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut boards = segments.map(parse_board).collect::<Vec<_>>();

    for number in numbers {
        let remaning_boards = boards.len();

        for board in boards.iter_mut() {
            if let Some((x, y)) = board.data.remove(&number) {
                board.hit_counter_x[x] += 1;
                if board.hit_counter_x[x] == 5 {
                    if remaning_boards == 1 {
                        return board.data.keys().sum::<i32>() * number;
                    }
                    board.delete = true;
                }

                board.hit_counter_y[y] += 1;
                if board.hit_counter_y[y] == 5 {
                    if remaning_boards == 1 {
                        return board.data.keys().sum::<i32>() * number;
                    }
                    board.delete = true;
                }
            }
        }

        if remaning_boards != 1 {
            boards = boards.into_iter().filter(|b| !b.delete).collect();
        }
    }

    println!("{:#?}", boards);

    0
}

fn main() {
    let input = get_input();
    let res = solve(&input);
    println!("{}", res);
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

        assert_eq!(crate::solve(input), 1924)
    }
}
