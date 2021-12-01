use std::{env, fs};

pub fn get_input() -> String {
    let program_name = env::current_exe().unwrap();
    let program_name = program_name.file_stem().unwrap().to_str().unwrap();
    let program_name = program_name.trim_end_matches('a');
    let program_name = program_name.trim_end_matches('b');

    fs::read_to_string(format!("input/{}.txt", program_name)).unwrap()
}
