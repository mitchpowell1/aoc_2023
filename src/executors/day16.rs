use super::Executor;

use std::{collections::VecDeque, fmt::Write};

use rayon::prelude::*;

#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty,
    NEMirror,
    SEMirror,
    NSSplitter,
    EWSplitter,
}

impl From<char> for Tile {
    fn from(c: char) -> Tile {
        match c {
            '.' => Tile::Empty,
            '/' => Tile::NEMirror,
            '\\' => Tile::SEMirror,
            '-' => Tile::EWSplitter,
            '|' => Tile::NSSplitter,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Hash, PartialEq, Clone, Copy, Eq)]
struct Point(isize, isize);

impl std::ops::Add<(i8, i8)> for Point {
    type Output = Point;
    fn add(self, other: (i8, i8)) -> Self {
        Point(self.0 + other.0 as isize, self.1 + other.1 as isize)
    }
}

impl Direction {
    fn get_offset(&self) -> (i8, i8) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
        }
    }

    fn get_visited_bitmask(&self) -> u8 {
        match self {
            Direction::North => 0b00000001,
            Direction::South => 0b00000010,
            Direction::East => 0b00000100,
            Direction::West => 0b00001000,
        }
    }

    fn get_new_directions(&self, tile: Tile) -> [Option<Direction>; 2] {
        use Direction::*;
        use Tile::*;
        match (self, tile) {
            (_, Empty) | (North | South, NSSplitter) | (East | West, EWSplitter) => {
                [Some(*self), None]
            }
            (North | South, EWSplitter) => [Some(East), Some(West)],
            (East | West, NSSplitter) => [Some(North), Some(South)],
            (North, NEMirror) => [Some(East), None],
            (South, NEMirror) => [Some(West), None],
            (East, NEMirror) => [Some(North), None],
            (West, NEMirror) => [Some(South), None],
            (North, SEMirror) => [Some(West), None],
            (South, SEMirror) => [Some(East), None],
            (East, SEMirror) => [Some(South), None],
            (West, SEMirror) => [Some(North), None],
        }
    }
}

#[derive(Default)]
pub struct Day16 {
    grid: Vec<Vec<Tile>>,
}

impl Day16 {
    fn in_bounds(&self, Point(i, j): Point) -> bool {
        i >= 0 && j >= 0 && (i as usize) < self.grid.len() && (j as usize) < self.grid[0].len()
    }

    fn get_num_energized(
        &self,
        starting_point: Point,
        starting_dir: Direction,
        to_visit: &mut VecDeque<(Direction, Point)>,
    ) -> u32 {
        to_visit.clear();

        let mut visited = [[0u8; 128]; 128];
        visited[starting_point.0 as usize][starting_point.1 as usize] |=
            starting_dir.get_visited_bitmask();
        to_visit.push_back((starting_dir, starting_point));

        while let Some((direction, point)) = to_visit.pop_front() {
            let i = point.0 as usize;
            let j = point.1 as usize;
            for next_direction in direction
                .get_new_directions(self.grid[i][j])
                .iter()
                .flatten()
            {
                let next_point = point + next_direction.get_offset();
                if !self.in_bounds(next_point) {
                    continue;
                }
                let next_i = next_point.0 as usize;
                let next_j = next_point.1 as usize;
                if (visited[next_i][next_j] & next_direction.get_visited_bitmask()) != 0 {
                    continue;
                }
                to_visit.push_back((*next_direction, next_point));
                visited[next_i][next_j] |= next_direction.get_visited_bitmask();
            }
        }

        visited.iter().flatten().filter(|&v| *v > 0).count() as u32
    }
}

impl Executor for Day16 {
    fn parse(&mut self, input: String) {
        let _input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        for line in input.lines() {
            self.grid.push(line.chars().map(Tile::from).collect());
        }
    }

    fn part_one(&mut self, output_buffer: &mut dyn Write) {
        let mut to_visit = VecDeque::default();
        let out = self.get_num_energized(Point(0, 0), Direction::East, &mut to_visit);

        _ = write!(output_buffer, "P1: {}", out)
    }

    fn part_two(&mut self, output_buffer: &mut dyn Write) {
        let max_out = (0..self.grid.len())
            .flat_map(|i| {
                [
                    (Point(i as isize, 0), Direction::East),
                    (
                        Point(i as isize, self.grid[0].len() as isize - 1),
                        Direction::West,
                    ),
                ]
            })
            .chain((0..self.grid[0].len()).flat_map(|i| {
                [
                    (Point(0, i as isize), Direction::South),
                    (
                        Point(self.grid.len() as isize - 1, i as isize),
                        Direction::North,
                    ),
                ]
            }))
            .par_bridge()
            .map_init(
                VecDeque::new,
                |to_visit, (point, dir)| self.get_num_energized(point, dir, to_visit),
            )
            .max()
            .unwrap();

        _ = write!(output_buffer, "P2: {max_out}");
    }
}
