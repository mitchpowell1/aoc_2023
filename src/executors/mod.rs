#![cfg_attr(rustfmt, rustfmt_skip)]
use std::fmt::Write;

pub trait Executor {
    fn parse(&mut self, input: String);
    fn part_one(&mut self, output_buffer: &mut dyn Write);
    fn part_two(&mut self, output_buffer: &mut dyn Write);
}

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

pub fn get_executors() -> Vec<Box<dyn Executor>> {
    vec![
        Box::<day1::Day1>::default(),
        Box::<day2::Day2>::default(),
        Box::<day3::Day3>::default(),
        Box::<day4::Day4>::default(),
        Box::<day5::Day5>::default(),
        Box::<day6::Day6>::default(),
        Box::<day7::Day7>::default(),
        Box::<day8::Day8>::default(),
        Box::<day9::Day9>::default(),
        Box::<day10::Day10>::default(),
        Box::<day11::Day11>::default(),
        Box::<day12::Day12>::default(),
        Box::<day13::Day13>::default(),
        Box::<day14::Day14>::default(),
        Box::<day15::Day15>::default(),
        Box::<day16::Day16>::default(),
        Box::<day17::Day17>::default(),
        Box::<day18::Day18>::default(),
        Box::<day19::Day19>::default(),
        Box::<day20::Day20>::default(),
        Box::<day21::Day21>::default(),
        Box::<day22::Day22>::default(),
        Box::<day23::Day23>::default(),
        Box::<day24::Day24>::default(),
        Box::<day25::Day25>::default(),
    ]
}

fn foo(
    bar: usize,
    baz: usize,
    whatever:usize,
    other: usize
    ) {}
