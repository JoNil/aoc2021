use std::{collections::HashMap, env, fmt::Display, fs};

use glam::{ivec2, IVec2};

pub fn get_input() -> String {
    let program_name = env::current_exe().unwrap();
    let program_name = program_name.file_stem().unwrap().to_str().unwrap();
    let program_name = program_name.trim_end_matches('a');
    let program_name = program_name.trim_end_matches('b');

    fs::read_to_string(format!("input/{}.txt", program_name)).unwrap()
}

pub fn print_map(map: &HashMap<IVec2, impl Display>) {
    let max_x = map
        .keys()
        .copied()
        .max_by(|p1, p2| p1.x.cmp(&p2.x))
        .unwrap();
    let max_y = map
        .keys()
        .copied()
        .max_by(|p1, p2| p1.y.cmp(&p2.y))
        .unwrap();

    for y in 0..=max_y.y {
        for x in 0..=max_x.x {
            let hit = map
                .get(&ivec2(x, y))
                .map(|count| format!("{}", count))
                .unwrap_or_else(|| ".".to_string());

            print!("{}", hit);
        }
        println!();
    }
}
