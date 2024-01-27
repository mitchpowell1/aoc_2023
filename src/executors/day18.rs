use nom::{
    bytes::complete::tag,
    character::complete::{hex_digit1, one_of},
    character::complete::{space1, u8},
    sequence::{delimited, preceded, tuple},
    IResult,
};

use crate::utils::{direction::Direction, point::Point};

use super::Executor;
use std::fmt::Write;

#[derive(Debug, Clone, Copy)]
struct Instruction {
    direction: Direction,
    count: u8,
    p2_count: u32,
    p2_direction: Direction,
}

impl Instruction {
    fn parse(input: &str) -> IResult<&str, Instruction> {
        let (input, (raw_direction, count, raw_color)) = tuple((
            one_of("LRUD"),
            preceded(space1, u8),
            preceded(
                space1,
                delimited(tag("("), preceded(tag("#"), hex_digit1), tag(")")),
            ),
        ))(input)?;
        let direction = match raw_direction {
            'L' => Direction::West,
            'R' => Direction::East,
            'U' => Direction::North,
            'D' => Direction::South,
            _ => unreachable!(),
        };
        let p2_count = u32::from_str_radix(&raw_color[..5], 16).unwrap();
        let p2_direction = match &raw_color.as_bytes()[5] {
            b'0' => Direction::East,
            b'1' => Direction::South,
            b'2' => Direction::West,
            b'3' => Direction::North,
            _ => unreachable!(),
        };
        Ok((
            input,
            Instruction {
                direction,
                count,
                p2_count,
                p2_direction,
            },
        ))
    }
}

#[derive(Default)]
pub struct Day18 {
    instructions: Vec<Instruction>,
}

impl Executor for Day18 {
    fn parse(&mut self, input: String) {
        for line in input.lines() {
            self.instructions.push(Instruction::parse(line).unwrap().1);
        }
    }

    fn part_one(&mut self, output_buffer: &mut dyn Write) {
        let vertices = self
            .instructions
            .iter()
            .scan((0i32, 0i32), |state, instruction| {
                let offset = instruction.direction.get_offset();
                state.0 += offset.0 as i32 * instruction.count as i32;
                state.1 += offset.1 as i32 * instruction.count as i32;
                Some(*state)
            });

        let total_area = compute_area(std::iter::once((0, 0)).chain(vertices));
        _ = write!(output_buffer, "P1: {}", total_area);
    }

    fn part_two(&mut self, output_buffer: &mut dyn Write) {
        let vertices = self
            .instructions
            .iter()
            .scan((0i32, 0i32), |state, instruction| {
                let offset = instruction.p2_direction.get_offset();
                state.0 += offset.0 as i32 * instruction.p2_count as i32;
                state.1 += offset.1 as i32 * instruction.p2_count as i32;
                Some(*state)
            });

        let total_area = compute_area(std::iter::once((0, 0)).chain(vertices));
        _ = write!(output_buffer, "P2: {}", total_area);
    }
}

fn compute_area(mut vertices: impl Iterator<Item = (i32, i32)>) -> i64 {
    let mut perimeter_area = 0;
    let mut shoelace_area = 0;
    let mut prev = vertices.next().unwrap();
    for v2 in vertices {
        let v1 = prev;
        perimeter_area += (v1.0 - v2.0).abs() as i64;
        perimeter_area += (v1.1 - v2.1).abs() as i64;
        shoelace_area += (v1.0 as i64 * v2.1 as i64) - (v1.1 as i64 * v2.0 as i64);
        prev = v2;
    }
    shoelace_area /= 2;
    let total_area = shoelace_area.abs() + ((perimeter_area / 2) + 1);
    total_area
}
