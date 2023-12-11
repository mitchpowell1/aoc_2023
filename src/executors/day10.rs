use super::Executor;

use std::collections::VecDeque;
use std::fmt::Write;

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn get_offset(&self) -> (i32, i32) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum TileType {
    Vertical,
    Horizontal,
    NorthEastBend,
    NorthWestBend,
    SouthWestBend,
    SouthEastBend,
    Ground,
    Start,
}

impl TileType {
    fn connects(&self, direction: Direction) -> bool {
        use Direction::*;
        use TileType::*;
        match (self, direction) {
            (Vertical, North | South) => true,
            (NorthWestBend, North | West) => true,
            (NorthEastBend, North | East) => true,
            (SouthWestBend, South | West) => true,
            (SouthEastBend, South | East) => true,
            (Horizontal, East | West) => true,
            _ => false,
        }
    }
}

impl TileType {
    fn from_char(c: char) -> Self {
        match c {
            '|' => TileType::Vertical,
            '-' => TileType::Horizontal,
            'L' => TileType::NorthEastBend,
            'J' => TileType::NorthWestBend,
            '7' => TileType::SouthWestBend,
            'F' => TileType::SouthEastBend,
            '.' => TileType::Ground,
            'S' => TileType::Start,
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }
}

#[derive(Debug)]
struct Tile(TileType, bool);

#[derive(Debug, Default)]
pub struct Day10 {
    tiles: Vec<Vec<Tile>>,
    start: (usize, usize),
}

impl Day10 {
    fn resolve_start_tile(&mut self) {
        let Self { tiles, start, .. } = self;
        let &mut (i, j) = start;
        // N, S, E, W
        let mut connections = [false; 4];
        for i_offset in -1..=1 {
            for j_offset in -1..=1 {
                if let 0 | 2 | -2 = i_offset + j_offset {
                    continue;
                }

                if out_of_bounds(tiles, (i, j), (i_offset, j_offset)) {
                    continue;
                }

                let neighbor_i = (i as i32 + i_offset) as usize;
                let neighbor_j = (j as i32 + j_offset) as usize;

                match (i_offset, j_offset, &tiles[neighbor_i][neighbor_j]) {
                    (-1, _, Tile(t, _)) if t.connects(Direction::South) => {
                        connections[0] = true;
                    }
                    (1, _, Tile(t, _)) if t.connects(Direction::North) => {
                        connections[1] = true;
                    }
                    (_, 1, Tile(t, _)) if t.connects(Direction::West) => {
                        connections[2] = true;
                    }
                    (_, -1, Tile(t, _)) if t.connects(Direction::East) => {
                        connections[3] = true;
                    }
                    _ => {}
                }
            }
        }

        debug_assert_eq!(
            connections
                .iter()
                .fold(0, |acc, &v| if v { acc + 1 } else { acc }),
            2
        );

        let Tile(tile_type, in_loop) = &mut tiles[i][j];

        *in_loop = true;
        *tile_type = match connections {
            [true, true, ..] => TileType::Vertical,
            [true, .., true, _] => TileType::NorthEastBend,
            [true, .., true] => TileType::NorthWestBend,
            [_, true, true, _] => TileType::SouthEastBend,
            [_, true, _, true] => TileType::SouthWestBend,
            _ => TileType::Horizontal,
        }
    }
}

impl Executor for Day10 {
    fn parse(&mut self, input: String) {
        for (i, line) in input.lines().enumerate() {
            let mut row = vec![];
            for (j, c) in line.chars().enumerate() {
                let tile_type = TileType::from_char(c);
                if tile_type == TileType::Start {
                    self.start = (i, j);
                }
                row.push(Tile(tile_type, false));
            }
            self.tiles.push(row);
        }
        self.resolve_start_tile();
    }

    fn part_one(&mut self, output_buffer: &mut dyn Write) {
        let mut max_depth = 0;
        let mut to_visit = VecDeque::new();
        let (i, j) = self.start;
        to_visit.push_back((i, j, 0));

        while let Some((i, j, depth)) = to_visit.pop_front() {
            max_depth = std::cmp::max(depth, max_depth);
            for direction in DIRECTIONS {
                if self.tiles[i][j].0.connects(direction) {
                    let (i_offset, j_offset) = direction.get_offset();
                    let next_i = (i as i32 + i_offset) as usize;
                    let next_j = (j as i32 + j_offset) as usize;
                    if !self.tiles[next_i][next_j].1 {
                        self.tiles[next_i][next_j].1 = true;
                        to_visit.push_back((next_i, next_j, depth + 1));
                    }
                }
            }
        }
        _ = write!(output_buffer, "P1: {max_depth}");
    }

    fn part_two(&mut self, output_buffer: &mut dyn Write) {
        let mut num_enclosed = 0;
        for row in self.tiles.iter() {
            let mut inside_loop = false;
            for tile in row.iter() {
                match tile {
                    Tile(t, true) if t.connects(Direction::North) => {
                        inside_loop = !inside_loop;
                    }
                    Tile(_, false) => {
                        if inside_loop {
                            num_enclosed += 1;
                        }
                    }
                    _ => {}
                }
            }
        }
        _ = write!(output_buffer, "P2 {num_enclosed}");
    }
}

fn out_of_bounds<T>(
    grid: &Vec<Vec<T>>,
    (i, j): (usize, usize),
    (i_offset, j_offset): (i32, i32),
) -> bool {
    let neighbor_i = i as i32 + i_offset;
    let neighbor_j = j as i32 + j_offset;
    neighbor_i < 0
        || neighbor_i >= grid.len() as i32
        || neighbor_j < 0
        || neighbor_j >= grid[0].len() as i32
}
