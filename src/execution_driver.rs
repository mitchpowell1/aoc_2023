use std::fs;
use std::time::Instant;

use crate::executors::Executor;

pub fn execute(mut executors: Vec<Box<dyn Executor>>, day: u8) {
    let executor = &mut executors[day as usize - 1];
    let input = fs::read_to_string(format!("inputs/day_{day}"))
        .expect("Encountered an error reading input file");
    let _global_pool = rayon::ThreadPoolBuilder::new().build_global();
    let mut p1_buffer = String::with_capacity(1024);
    let mut p2_buffer = String::with_capacity(1024);
    let start = Instant::now();
    executor.parse(input);
    let parse_time = start.elapsed();
    executor.part_one(&mut p1_buffer);
    let p1_time = start.elapsed();
    executor.part_two(&mut p2_buffer);
    let p2_time = start.elapsed();

    println!("Parse time: {:?}", parse_time);
    println!("Part one time: {:?}", p1_time - parse_time);
    println!("Part two time: {:?}", p2_time - p1_time);
    println!("Total Time: {:?}", p2_time);
    println!("{}", p1_buffer);
    println!("{}", p2_buffer);
}
