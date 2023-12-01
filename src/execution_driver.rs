use std::fs;
use std::time::Instant;

use crate::executors::Executor;

pub fn execute(mut executors: Vec<Box<dyn Executor>>, day: u8) {
    let executor = &mut executors[day as usize - 1];
    let input = fs::read_to_string(format!("inputs/day_{day}"))
        .expect("Encountered an error reading input file");
    let start = Instant::now();
    executor.parse(input);
    let parse_time = start.elapsed();
    executor.part_one();
    let p1_time = start.elapsed();
    executor.part_two();
    let p2_time = start.elapsed();

    println!("\nParse time: {:?}", parse_time);
    println!("Part one time: {:?}", p1_time - parse_time);
    println!("Part two time: {:?}", p2_time - p1_time);
    println!("Total {:?}", p2_time);
}
