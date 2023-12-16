use super::Executor;

use std::fmt::Write;

#[derive(Default)]
pub struct Day15 {
    input: String,
}

#[derive(Debug, Clone, Copy)]
pub struct Lens<'a> {
    label: &'a str,
    focal_power: u32,
}

fn run_hash_algorithm(s: &str) -> usize {
    let mut current_value = 0;
    for b in s.as_bytes() {
        current_value += *b as usize;
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}

impl Executor for Day15 {
    fn parse(&mut self, input: String) {
        self.input = input;
    }

    fn part_one(&mut self, output_buffer: &mut dyn Write) {
        let out: usize = self.input.trim().split(',').map(run_hash_algorithm).sum();
        _ = write!(output_buffer, "P1: {}", out);
    }

    fn part_two(&mut self, output_buffer: &mut dyn Write) {
        let mut boxes: [Vec<Lens>; 256] = std::array::from_fn(|_| vec![]);
        for instruction in self.input.trim().split(',') {
            let op_idx = instruction.find(|c| c == '-' || c == '=').unwrap();
            let label = &instruction[..op_idx];
            let raw_focal_power = &instruction[op_idx + 1..];
            let lens_box = run_hash_algorithm(label);
            let lens_position = boxes[lens_box].iter().position(|&l| l.label == label);
            match &instruction[op_idx..=op_idx] {
                "-" => {
                    if let Some(index) = lens_position {
                        boxes[lens_box].remove(index);
                    }
                }
                "=" => {
                    let focal_power = raw_focal_power.parse().unwrap();
                    match lens_position {
                        Some(index) => boxes[lens_box][index].focal_power = focal_power,
                        None => boxes[lens_box].push(Lens { label, focal_power }),
                    }
                }
                _ => unreachable!(),
            }
        }

        let out: usize = boxes
            .iter()
            .enumerate()
            .map(|(box_num, l_box)| {
                l_box
                    .iter()
                    .enumerate()
                    .map(|(lens_num, lens)| {
                        (1 + box_num) * (1 + lens_num) * lens.focal_power as usize
                    })
                    .sum::<usize>()
            })
            .sum();
        _ = write!(output_buffer, "P2: {out}");
    }
}
