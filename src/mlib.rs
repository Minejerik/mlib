
use rand::{distributions::Alphanumeric, Rng};
use std::env;
use std::fs;
use std::io::Write;

pub fn mkdir(path: String) {
    let _ = fs::create_dir(path);
}

pub fn gen_random_string(length: i32) -> String {
    let rand_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length as usize)
        .map(char::from)
        .collect();

    rand_string
}

pub fn get_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();

    args
}

pub fn make_and_write_file(path: String, contents: String) {
    let mut file = fs::File::create(path).unwrap();

    file.write_all(contents.as_bytes()).unwrap();
}
