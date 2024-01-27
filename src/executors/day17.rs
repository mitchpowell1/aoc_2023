use super::Executor;
use crate::utils::direction::Direction;
use crate::utils::point::*;

use std::collections::VecDeque;
use std::fmt::Write;

#[derive(Default)]
pub struct Day17 {
    heat_map: Vec<Vec<u8>>,
}

impl Direction {
    fn visit_number(&self) -> u8 {
        match self {
            Direction::North | Direction::South => 0b00000001,
            Direction::East | Direction::West => 0b00000010,
        }
    }
}

impl Day17 {
    fn get_min_cost(&self, turning_scheme: TurningScheme) -> u32 {
        let mut to_visit = BucketQueue::new();
        let mut visited = [[[0; 16]; 256]; 256];
        let target = Point(
            self.heat_map.len() as i32 - 1,
            self.heat_map[0].len() as i32 - 1,
        );

        to_visit.push(TraversalState {
            location: Point(0, 0),
            current_streak: 0,
            cost: 0,
            direction: Direction::East,
        });

        to_visit.push(TraversalState {
            location: Point(0, 0),
            current_streak: 0,
            cost: 0,
            direction: Direction::South,
        });

        loop {
            let t = to_visit.pop().expect("Ran out of nodes to visit before we reached the target. Something went horribly wrong");
            let TraversalState {
                location,
                current_streak,
                cost,
                direction,
            } = t;
            match turning_scheme {
                TurningScheme::Crucible => {
                    if location == target {
                        break cost;
                    }
                }
                TurningScheme::UltraCrucible => {
                    if location == target && current_streak > 3 {
                        break cost;
                    }
                }
            }
            for next_direction in t
                .possible_next_directions(turning_scheme)
                .into_iter()
                .flatten()
            {
                let next_location = location + next_direction;
                if !self.heat_map.is_in_bounds(next_location) {
                    continue;
                }
                let next_streak = if next_direction == direction {
                    current_streak + 1
                } else {
                    1
                };

                if visited[next_location.0 as usize][next_location.1 as usize][next_streak as usize]
                    & next_direction.visit_number()
                    != 0
                {
                    continue;
                }

                let next_cost =
                    cost + self.heat_map[next_location.0 as usize][next_location.1 as usize] as u32;

                visited[next_location.0 as usize][next_location.1 as usize]
                    [next_streak as usize] |= next_direction.visit_number();

                to_visit.push(TraversalState {
                    location: next_location,
                    current_streak: next_streak,
                    cost: next_cost,
                    direction: next_direction,
                })
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct TraversalState {
    cost: u32,
    location: Point,
    current_streak: u8,
    direction: Direction,
}

#[derive(Copy, Clone)]
enum TurningScheme {
    Crucible,
    UltraCrucible,
}

impl TraversalState {
    fn possible_next_directions(&self, turning_scheme: TurningScheme) -> [Option<Direction>; 3] {
        use Direction::*;
        if let TurningScheme::Crucible = turning_scheme {
            match (self.direction, self.current_streak) {
                (North | South, 3) => [Some(East), Some(West), None],
                (direction @ (North | South), _) => [Some(East), Some(West), Some(direction)],
                (East | West, 3) => [Some(North), Some(South), None],
                (direction @ (East | West), _) => [Some(North), Some(South), Some(direction)],
            }
        } else {
            match (self.direction, self.current_streak) {
                (d, 0..=3) => [Some(d), None, None],
                (North | South, 10) => [Some(East), Some(West), None],
                (direction @ (North | South), _) => [Some(East), Some(West), Some(direction)],
                (East | West, 10) => [Some(North), Some(South), None],
                (direction @ (East | West), _) => [Some(North), Some(South), Some(direction)],
            }
        }
    }
}

impl PartialOrd for TraversalState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TraversalState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl Executor for Day17 {
    fn parse(&mut self, input: String) {
        for line in input.lines() {
            self.heat_map.push(
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect(),
            )
        }
    }

    fn part_one(&mut self, output_buffer: &mut dyn Write) {
        let target_distance = self.get_min_cost(TurningScheme::Crucible);
        _ = write!(output_buffer, "P1: {target_distance}");
    }

    fn part_two(&mut self, output_buffer: &mut dyn Write) {
        let target_distance = self.get_min_cost(TurningScheme::UltraCrucible);
        _ = write!(output_buffer, "P2: {target_distance}");
    }
}

struct BucketQueue {
    buckets: [VecDeque<TraversalState>; 300 * 9],
    front: Option<usize>,
}

impl BucketQueue {
    fn new() -> BucketQueue {
        BucketQueue {
            buckets: std::array::from_fn(|_| VecDeque::new()),
            front: None,
        }
    }

    fn find_front(&self) -> Option<usize> {
        if let Some(b) = self.front {
            for i in b..self.buckets.len() {
                if !self.buckets[i].is_empty() {
                    return Some(i);
                }
            }
        }
        None
    }

    fn push(&mut self, t: TraversalState) {
        let bucket = t.cost as usize;
        self.buckets[bucket].push_back(t);

        match self.front {
            None => self.front = Some(bucket),
            Some(t) if t > bucket => self.front = Some(bucket),
            _ => {}
        }
    }

    fn pop(&mut self) -> Option<TraversalState> {
        match self.front {
            Some(b) => {
                let out = self.buckets[b].pop_front();
                self.front = self.find_front();
                out
            }
            None => None,
        }
    }
}
