use rayon::prelude::*;

use super::Executor;
use std::fmt::Write;

const P1_EXPANSION_FACTOR: usize = 2;
const P2_EXPANSION_FACTOR: usize = 1_000_000;

#[derive(Default, Debug)]
pub struct Day11 {
    populated_column_counts: Vec<u8>,
    populated_row_counts: Vec<u8>,
    stars: Vec<(u8, u8)>,
    p2_result: usize,
}

impl Day11 {
    fn get_empty_rows_between(&self, smaller_row: u8, larger_row: u8) -> u8 {
        (larger_row - smaller_row)
            - (self.populated_row_counts[larger_row as usize]
                - self.populated_row_counts[smaller_row as usize])
    }

    fn get_empty_cols_between(&self, smaller_col: u8, larger_col: u8) -> u8 {
        (larger_col - smaller_col)
            - (self.populated_column_counts[larger_col as usize]
                - self.populated_column_counts[smaller_col as usize])
    }

    fn get_distance(&self, (r1, c1): (u8, u8), (r2, c2): (u8, u8)) -> (usize, usize) {
        let base_distance = (r2 - r1) as usize + (c1 as isize - c2 as isize).unsigned_abs();
        let (min_c, max_c) = (std::cmp::min(c1, c2), std::cmp::max(c1, c2));
        let num_empty_rows = self.get_empty_rows_between(r1, r2) as usize;
        let num_empty_cols = self.get_empty_cols_between(min_c, max_c) as usize;

        let mut p1_expansion_distance = (num_empty_rows * P1_EXPANSION_FACTOR) - num_empty_rows;
        p1_expansion_distance += (num_empty_cols * P1_EXPANSION_FACTOR) - num_empty_cols;

        let mut p2_expansion_distance = (num_empty_rows * P2_EXPANSION_FACTOR) - num_empty_rows;
        p2_expansion_distance += (num_empty_cols * P2_EXPANSION_FACTOR) - num_empty_cols;

        (
            base_distance + p1_expansion_distance,
            base_distance + p2_expansion_distance,
        )
    }
}

impl Executor for Day11 {
    fn parse(&mut self, input: String) {
        for _ in input.lines().next().unwrap().as_bytes() {
            self.populated_column_counts.push(0);
        }

        for (i, line) in input.lines().enumerate() {
            let mut row_populated = 0;
            for (j, c) in line.chars().enumerate() {
                if c == '#' {
                    row_populated = 1;
                    self.stars.push((i as u8, j as u8));
                    self.populated_column_counts[j] = 1;
                }
            }
            self.populated_row_counts.push(row_populated);
        }
        let mut populated_so_far = 0;
        for row_count in self.populated_row_counts.iter_mut() {
            *row_count += populated_so_far;
            populated_so_far = *row_count
        }
        populated_so_far = 0;
        for col_count in self.populated_column_counts.iter_mut() {
            *col_count += populated_so_far;
            populated_so_far = *col_count
        }
    }

    fn part_one(&mut self, output_buffer: &mut dyn Write) {
        let (p1_total, p2_total) = self
            .stars
            .par_iter()
            .enumerate()
            .map(|(i, s1)| {
                self.stars[i + 1..]
                    .par_iter()
                    .map(|s2| self.get_distance(*s1, *s2))
                    .reduce(
                        || (0, 0),
                        |(p1_acc, p2_acc), (p1, p2)| (p1 + p1_acc, p2 + p2_acc),
                    )
            })
            .reduce(
                || (0, 0),
                |(p1_acc, p2_acc), (p1, p2)| (p1 + p1_acc, p2 + p2_acc),
            );
        self.p2_result = p2_total;
        _ = write!(output_buffer, "P1: {p1_total}");
    }

    fn part_two(&mut self, output_buffer: &mut dyn Write) {
        _ = write!(output_buffer, "P2: {}", self.p2_result);
    }
}
