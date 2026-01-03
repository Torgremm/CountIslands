mod create;
mod solve;
use std::{collections::HashMap, path::PathBuf, time::Instant};

fn main() {
    let answers: HashMap<usize, u8> = create::create_tests();
    let start = Instant::now();
    for n in 1..=answers.len() {
        execute_test(n, &answers);
    }

    let t = start.elapsed();
    println!("Elapsed: {:?}", t);
}

fn execute_test(n: usize, answers: &HashMap<usize, u8>) {
    let file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test")
        .join(format!("Test{}.txt", n));

    let input = std::fs::read_to_string(&file);
    let result = solve::count_islands(input.unwrap());
    assert_eq!(result, *answers.get(&(n)).unwrap());
}
