use rustc_hash::{FxHashMap, FxHashSet};

use crate::utils::direction::*;
use crate::utils::point::{InBounds, Point};

use super::Executor;

use std::collections::VecDeque;
use std::fmt::Write;

// const P1_STEPS: u8 = 64;
const P1_STEPS: u8 = 7;
// const P2_STEPS: u32 = 26_501_365;
const P2_STEPS: u32 = 7;

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::East,
    Direction::West,
];

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Garden,
    Rock,
}

impl From<char> for Tile {
    fn from(c: char) -> Tile {
        match c {
            '#' => Tile::Rock,
            '.' | 'S' => Tile::Garden,
            _ => unreachable!(),
        }
    }
}
#[derive(Default)]
pub struct Day21 {
    tiles: Vec<Vec<Tile>>,
    start: (usize, usize),
}

impl Executor for Day21 {
    fn parse(&mut self, input: String) {
        let _input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";
        for (i, line) in input.lines().enumerate() {
            let mut row = vec![];
            for (j, c) in line.chars().enumerate() {
                if c == 'S' {
                    self.start = (i, j);
                }
                row.push(Tile::from(c));
            }
            self.tiles.push(row);
        }
        dbg!(&self.tiles.len(), &self.tiles[0].len());
    }

    fn part_one(&mut self, output_buffer: &mut dyn Write) {
        let mut to_visit = VecDeque::new();
        let mut visited = FxHashSet::default();

        let start = (Point(self.start.0 as i32, self.start.1 as i32), 0);
        let mut count = 0;
        to_visit.push_back(start);
        visited.insert(start);
        while let Some((p, steps)) = to_visit.pop_back() {
            if steps == P1_STEPS {
                // println!("{p:?}");
                count += 1;
                continue;
            }

            for direction in DIRECTIONS {
                let next_p = p + direction;
                let next_steps = steps + 1;
                let next = (next_p, next_steps);
                if self.tiles.is_in_bounds(next_p)
                    && !visited.contains(&next)
                    && self.tiles[next_p.0 as usize][next_p.1 as usize] != Tile::Rock
                {
                    visited.insert(next);
                    to_visit.push_back(next);
                }
            }
        }

        _ = write!(output_buffer, "P1: {count:?}");
    }

    fn part_two(&mut self, output_buffer: &mut dyn Write) {
        let mut currently_visiting = FxHashMap::default();
        let mut to_visit = FxHashMap::default();
        let mut buffer = vec![];

        let start = Point(self.start.0 as i32, self.start.1 as i32);
        // let mut count = 0;
        currently_visiting.insert(start, 1);
        for _i in 0..P2_STEPS {
            for (point, multiples) in currently_visiting.drain() {
                for direction in DIRECTIONS {
                    let next_p = point + direction;
                    let mapped = Point(
                        next_p.0.rem_euclid(self.tiles.len() as i32 - 1),
                        next_p.1.rem_euclid(self.tiles[0].len() as i32 - 1),
                    );
                    if !to_visit.contains_key(&next_p)
                        && self.tiles[mapped.0 as usize][mapped.1 as usize] != Tile::Rock
                    {
                        to_visit.insert(next_p, multiples);
                    }
                }
            }
            for (k, v) in to_visit.drain() {
                let mapped = Point(
                    k.0.rem_euclid(self.tiles.len() as i32 - 1),
                    k.1.rem_euclid(self.tiles[0].len() as i32 - 1),
                );
                buffer.push((mapped, v));
            }
            for (mapped_point, multiples) in buffer.drain(..) {
                to_visit
                    .entry(mapped_point)
                    .and_modify(|v| *v += multiples)
                    .or_insert(multiples);
            }
            std::mem::swap(&mut to_visit, &mut currently_visiting);
        }
        // println!(&currently_visiting, &to_visit);
        // for p in &currently_visiting {
        //     println!("{p:?}");
        // }
        let s = currently_visiting.values().sum::<u64>();
        _ = write!(output_buffer, "P2: {s:?}");
    }
}
