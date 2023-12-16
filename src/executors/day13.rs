use super::Executor;
use std::fmt::Write;

#[derive(Default)]
pub struct Day13 {
    input: Vec<Vec<Vec<u8>>>,
}

fn get_horizontal_line_of_reflection(input: &Vec<Vec<u8>>, num_smudges: i32) -> Option<usize> {
    for divider in 1..input[0].len() {
        let mut smudges = 0;
        for line in input {
            let mut j = divider as isize - 1;
            let mut k = divider as isize;
            while j >= 0 && k < line.len() as isize {
                if line[j as usize] != line[k as usize] {
                    smudges += 1;
                }
                j -= 1;
                k += 1;
            }
        }
        if smudges == num_smudges {
            return Some(divider);
        }
    }

    None
}

fn get_vertical_line_of_reflection(input: &Vec<Vec<u8>>, num_smudges: i32) -> Option<usize> {
    for divider in 1..input.len() {
        let mut smudges = 0;
        for i in 0..input[0].len() {
            let mut j = divider as isize - 1;
            let mut k = divider as isize;
            while j >= 0 && k < input.len() as isize {
                if input[j as usize][i] != input[k as usize][i] {
                    smudges += 1;
                }
                j -= 1;
                k += 1;
            }
        }
        if smudges == num_smudges {
            return Some(divider);
        }
    }
    None
}

impl Executor for Day13 {
    fn parse(&mut self, input: String) {
        self.input = input
            .split("\n\n")
            .map(|s| {
                s.lines()
                    .map(|l| l.as_bytes().iter().map(|&b| b).collect())
                    .collect()
            })
            .collect()
    }

    fn part_one(&mut self, output_buffer: &mut dyn Write) {
        let out = self
            .input
            .iter()
            .map(|i| {
                100 * get_vertical_line_of_reflection(i, 0).unwrap_or_default()
                    + get_horizontal_line_of_reflection(i, 0).unwrap_or_default()
            })
            .sum::<usize>();
        _ = write!(output_buffer, "P1: {out}");
    }

    fn part_two(&mut self, output_buffer: &mut dyn Write) {
        let out = self
            .input
            .iter()
            .map(|i| {
                100 * get_vertical_line_of_reflection(i, 1).unwrap_or_default()
                    + get_horizontal_line_of_reflection(i, 1).unwrap_or_default()
            })
            .sum::<usize>();
        _ = write!(output_buffer, "P2: {out}");
    }
}
