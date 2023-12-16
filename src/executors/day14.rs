use rustc_hash::FxHashSet;

use super::Executor;
use std::fmt::Write;

const P2_CYCLE_NUM: u32 = 1_000_000_000;
#[derive(Clone, PartialEq, Hash, Eq)]
pub enum Tile {
    Rock,
    Cube,
    Empty,
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Rock => 'O',
                Tile::Cube => '#',
                Tile::Empty => '.',
            },
        )
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Tile {
        match c {
            '.' => Tile::Empty,
            'O' => Tile::Rock,
            '#' => Tile::Cube,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn offset(&self) -> (isize, isize) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
        }
    }

    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Default, Debug, Clone, Hash, PartialEq)]
pub struct Platform {
    rocks: Vec<Vec<Tile>>,
    digest: Option<[u128; 128]>,
}

impl Platform {
    fn in_bounds(&self, (i, j): (isize, isize)) -> bool {
        i >= 0 && j >= 0 && (i as usize) < self.rocks.len() && (j as usize) < self.rocks[0].len()
    }
    fn tilt(&mut self, direction: Direction) {
        let Self { rocks, .. } = self;
        let search_offset = direction.opposite().offset();
        let scan_offset = match direction {
            Direction::North | Direction::South => Direction::East.offset(),
            _ => Direction::South.offset(),
        };
        let mut starting_location: (isize, isize) = match direction {
            Direction::North => (0, 0),
            Direction::East => (0, rocks[0].len() as isize - 1),
            Direction::South => (rocks.len() as isize - 1, 0),
            Direction::West => (0, 0),
        };

        while self.in_bounds(starting_location) {
            let mut fall_location = starting_location;
            while self.in_bounds(fall_location) {
                let (i, j) = &mut fall_location;
                if self.rocks[*i as usize][*j as usize] == Tile::Empty {
                    break;
                }
                fall_location.0 += search_offset.0;
                fall_location.1 += search_offset.1;
            }
            let mut search_location = fall_location;
            let mut encountered_cube = false;
            while self.in_bounds(search_location) {
                let (i, j) = search_location;
                let digest = self.digest.iter_mut().next().unwrap();
                match self.rocks[i as usize][j as usize] {
                    Tile::Rock => {
                        if !encountered_cube {
                            self.rocks[i as usize][j as usize] = Tile::Empty;
                            self.rocks[fall_location.0 as usize][fall_location.1 as usize] =
                                Tile::Rock;
                            digest[i as usize] &= !(1 << j);
                            digest[fall_location.0 as usize] |= 1 << fall_location.1;
                            fall_location.0 += search_offset.0;
                            fall_location.1 += search_offset.1;
                        }
                    }
                    Tile::Empty => {
                        if encountered_cube {
                            encountered_cube = false;
                            fall_location = search_location;
                        }
                    }
                    Tile::Cube => {
                        encountered_cube = true;
                    }
                }
                search_location.0 += search_offset.0;
                search_location.1 += search_offset.1;
            }
            starting_location.0 += scan_offset.0;
            starting_location.1 += scan_offset.1;
        }
    }

    fn calculate_load(&self) -> usize {
        let mut total = 0;
        for i in 0..self.rocks.len() {
            for j in 0..self.rocks[0].len() {
                if let Tile::Rock = self.rocks[i][j] {
                    let height = self.rocks.len() - i;
                    total += height
                }
            }
        }
        total
    }

    fn compute_digest(&mut self) {
        let mut out = [0u128; 128];
        for (i, row) in self.rocks.iter().enumerate() {
            for j in 0..row.len() {
                if self.rocks[i][j] == Tile::Rock {
                    out[i] |= 1 << j;
                }
            }
        }
        self.digest = Some(out);
    }

    fn get_rock_digest(&self) -> [u128; 128] {
        unsafe { self.digest.unwrap_unchecked() }
        // let mut out = [0u128; 128];
        // for (i, row) in self.rocks.iter().enumerate() {
        //     for j in 0..row.len() {
        //         if self.rocks[i][j] == Tile::Rock {
        //             out[i] |= 1 << j;
        //         }
        //     }
        // }
        // out
    }
}

#[derive(Default)]
pub struct Day14 {
    platform: Platform,
}

impl Executor for Day14 {
    fn parse(&mut self, input: String) {
        let _input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let mut rocks = vec![];
        for line in input.lines() {
            rocks.push(line.chars().map(Tile::from).collect())
        }
        let mut platform = Platform {
            rocks,
            digest: None,
        };
        platform.compute_digest();
        self.platform = platform;
    }

    fn part_one(&mut self, output_buffer: &mut dyn Write) {
        self.platform.tilt(Direction::North);
        _ = write!(output_buffer, "P1: {}", self.platform.calculate_load());
    }

    fn part_two(&mut self, output_buffer: &mut dyn Write) {
        let tilts = [
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ];

        let mut previous_tile_states = FxHashSet::default();
        let mut cycles = 0;
        let mut start_cycle = 0;
        let mut encountered = false;
        // Skip 1 because we already tilted everything North in part one and
        // we don't want to clone everything unnecessarily
        for &t in tilts.iter().cycle().skip(1) {
            self.platform.tilt(t);
            if t == Direction::East {
                cycles += 1;
                let digest = &self.platform.get_rock_digest();
                if previous_tile_states.contains(digest) {
                    if encountered {
                        break;
                    }
                    encountered = true;
                    start_cycle = cycles;
                    previous_tile_states.clear();
                }
                previous_tile_states.insert(*digest);
            }
        }
        let cycle_length = cycles - start_cycle;

        let remaining_cycles = (P2_CYCLE_NUM - cycles) % cycle_length;
        for _ in 0..remaining_cycles {
            for t in tilts {
                self.platform.tilt(t);
            }
        }
        _ = write!(output_buffer, "P2: {}", self.platform.calculate_load());
    }
}
