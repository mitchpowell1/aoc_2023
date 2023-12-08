use super::Executor;

use std::fmt::Write;

#[derive(Default, Debug)]
pub struct Day9 {
    histories: Vec<Vec<i32>>,
    value_buffer1: Vec<i32>,
    value_buffer2: Vec<i32>,
    first_last_value_buffer: Vec<(i32, i32)>,
}

fn produce_sequence_value<F>(
    current_sequence: &mut Vec<i32>,
    next_sequence: &mut Vec<i32>,
    first_last_values: &mut Vec<(i32, i32)>,
    aggregator: F,
) -> i32
where
    F: Fn(i32, (i32, i32)) -> i32,
{
    let mut zero_check = current_sequence.iter().fold(0, |a, b| a | b);
    while zero_check != 0 {
        zero_check = 0;
        first_last_values.push((
            current_sequence[0],
            current_sequence[current_sequence.len() - 1],
        ));
        next_sequence.clear();
        for w in current_sequence.windows(2) {
            let v = w[1] - w[0];
            zero_check |= v;
            next_sequence.push(v);
        }
        std::mem::swap(current_sequence, next_sequence);
    }
    first_last_values.drain(..).rev().fold(0, aggregator)
}

impl Executor for Day9 {
    fn parse(&mut self, input: String) {
        input.lines().for_each(|s| {
            let history = s.split_whitespace().map(|s| s.parse().unwrap()).collect();
            self.histories.push(history)
        });
    }

    fn part_one(&mut self, output_buffer: &mut dyn Write) {
        let aggregator = |acc, (_, last)| acc + last;
        let v: i32 = self
            .histories
            .iter()
            .map(|h| {
                h.clone_into(&mut self.value_buffer1);
                produce_sequence_value(
                    &mut self.value_buffer1,
                    &mut self.value_buffer2,
                    &mut self.first_last_value_buffer,
                    aggregator,
                )
            })
            .sum();

        _ = write!(output_buffer, "P1: {v}");
    }

    fn part_two(&mut self, output_buffer: &mut dyn Write) {
        let aggregator = |acc, (first, _)| first - acc;
        let v: i32 = self
            .histories
            .iter()
            .map(|h| {
                h.clone_into(&mut self.value_buffer1);
                produce_sequence_value(
                    &mut self.value_buffer1,
                    &mut self.value_buffer2,
                    &mut self.first_last_value_buffer,
                    aggregator,
                )
            })
            .sum();

        _ = write!(output_buffer, "P2: {v}");
    }
}
