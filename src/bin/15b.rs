use aoc2021::{get_input, parse_map};
use glam::{const_ivec2, ivec2, IVec2};
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

static DIRECTIONS: &[IVec2] = &[
    const_ivec2!([-1, 0]),
    const_ivec2!([1, 0]),
    const_ivec2!([0, -1]),
    const_ivec2!([0, 1]),
];

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    pos: IVec2,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.pos.x.cmp(&other.pos.x))
            .then_with(|| self.pos.y.cmp(&other.pos.y))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn reconstruct_path(came_from: &HashMap<IVec2, IVec2>, mut current: IVec2) -> Vec<IVec2> {
    let mut total_path = vec![current];
    while let Some(new) = came_from.get(&current) {
        current = *new;
        total_path.insert(0, current);
    }
    total_path
}

fn solve(input: &str) -> i32 {
    let mut map = parse_map(input, |c| c.to_digit(10).unwrap() as i32);

    let max_x = map.keys().map(|v| v.x).max().unwrap();
    let max_y = map.keys().map(|v| v.y).max().unwrap();

    let width = max_x + 1;
    let height = max_y + 1;

    for tile_y in 0..5 {
        for tile_x in 0..5 {
            if tile_x == 0 && tile_y == 0 {
                continue;
            }

            for y in 0..height {
                for x in 0..width {
                    let new_x = width * tile_x + x;
                    let new_y = height * tile_y + y;

                    let orig = *map.get(&ivec2(x, y)).unwrap();

                    let val = ((orig - 1) + tile_x + tile_y) % 9 + 1;

                    map.insert(ivec2(new_x, new_y), val);
                }
            }
        }
    }

    let max_x = map.keys().map(|v| v.x).max().unwrap();
    let max_y = map.keys().map(|v| v.y).max().unwrap();

    let start = ivec2(0, 0);
    let end = ivec2(max_x, max_y);

    let h = |pos| {
        let diff: IVec2 = pos - end;
        diff.x.abs() + diff.y.abs()
    };

    let mut open_set = BinaryHeap::new();
    open_set.push(State {
        cost: 0,
        pos: start,
    });

    let mut came_from = HashMap::new();

    let mut g_score = HashMap::new();
    *g_score.entry(start).or_default() = 0;

    let mut f_score = HashMap::new();
    *f_score.entry(start).or_default() = h(start);

    let mut total_path = Vec::new();

    while let Some(State {
        cost: _,
        pos: current,
    }) = open_set.pop()
    {
        if current == ivec2(max_x, max_y) {
            total_path = reconstruct_path(&came_from, current);
            break;
        }

        for dir in DIRECTIONS {
            let neighbor = current + *dir;

            if let Some(d) = map.get(&neighbor) {
                let tentative_g_score = *g_score.entry(current).or_insert(i32::MAX) + *d;

                if tentative_g_score < *g_score.entry(neighbor).or_insert(i32::MAX) {
                    let f = tentative_g_score + h(neighbor);
                    came_from.insert(neighbor, current);
                    g_score.insert(neighbor, tentative_g_score);
                    f_score.insert(neighbor, *d);
                    open_set.push(State {
                        cost: f,
                        pos: neighbor,
                    });
                }
            }
        }
    }

    total_path
        .iter()
        .map(|pos| *map.get(pos).unwrap())
        .sum::<i32>()
        - *map.get(&start).unwrap()
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
        let input = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
        assert_eq!(crate::solve(input), 315)
    }
}
