use rayon::prelude::*;

use rustc_hash::{FxHashMap, FxHashSet};

use super::Executor;

use nom::{
    bytes::complete::tag,
    character::complete::alphanumeric1,
    sequence::{delimited, separated_pair, terminated, tuple},
    IResult,
};

#[derive(Debug)]
enum Direction {
    Right,
    Left,
}

impl Direction {
    fn from_char(c: char) -> Direction {
        match c {
            'R' => Direction::Right,
            'L' => Direction::Left,
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }
}

#[derive(Default, Debug)]
pub struct Day8 {
    directions: Vec<Direction>,
    map: Vec<(usize, usize)>,
    a_nodes: Vec<usize>,
    z_nodes: Vec<usize>,
    start: usize,
    end: usize,
}

fn parse_node(input: &str) -> IResult<&str, (&str, &str, &str)> {
    let (input, (node, (left, right))) = tuple((
        terminated(alphanumeric1, tag(" = ")),
        delimited(
            tag("("),
            separated_pair(alphanumeric1, tag(", "), alphanumeric1),
            tag(")"),
        ),
    ))(input)?;

    Ok((input, (node, left, right)))
}

impl Executor for Day8 {
    fn parse(&mut self, input: String) {
        let mut sections = input.split("\n\n");
        let directions: Vec<Direction> = sections
            .next()
            .unwrap()
            .chars()
            .map(Direction::from_char)
            .collect();

        let nodes = sections.next().unwrap();
        let mut node_vec = vec![];
        let mut a_nodes = vec![];
        let mut z_nodes = vec![];
        let mut node_indexes = FxHashMap::default();
        let mut start = 0;
        let mut end = 0;
        for (i, line) in nodes.lines().enumerate() {
            let (_, (node, left, right)) = parse_node(line).unwrap();
            match node {
                "AAA" => start = i,
                "ZZZ" => end = i,
                _ => {}
            }

            match node.as_bytes().last() {
                Some(b'A') => a_nodes.push(i),
                Some(b'Z') => z_nodes.push(i),
                _ => {}
            }

            node_indexes.insert(node, i);
            node_vec.push((left, right));
        }
        let map: Vec<_> = node_vec
            .iter()
            .map(|(l, r)| (*node_indexes.get(l).unwrap(), *node_indexes.get(r).unwrap()))
            .collect();
        self.map = map;
        self.directions = directions;
        self.start = start;
        self.end = end;
        self.a_nodes = a_nodes;
        self.z_nodes = z_nodes;
    }

    fn part_one(&mut self) {
        let Day8 {
            directions,
            map,
            start,
            end,
            ..
        } = &self;
        let mut direction_index = 0;
        let mut current_node = *start;
        let mut num_steps = 0;

        while current_node != *end {
            match directions[direction_index] {
                Direction::Left => current_node = map[current_node].0,
                Direction::Right => current_node = map[current_node].1,
            }
            num_steps += 1;
            direction_index = (direction_index + 1) % directions.len();
        }
        println!("P1: {num_steps}");
    }

    fn part_two(&mut self) {
        let Day8 {
            directions,
            map,
            a_nodes,
            z_nodes,
            ..
        } = self;
        let min_path_length = a_nodes
            .par_iter()
            .fold(
                || 1,
                |mut min_path_length, node| {
                    let mut visited = FxHashSet::default();
                    let mut current_node = *node;
                    let mut path_length = 0u64;
                    let mut direction_index = 1;
                    while !visited.contains(&(current_node, direction_index)) {
                        path_length += 1;
                        visited.insert((current_node, direction_index));

                        current_node = match directions[direction_index] {
                            Direction::Left => map[current_node].0,
                            Direction::Right => map[current_node].1,
                        };

                        direction_index = (direction_index + 1) % directions.len();

                        if z_nodes.contains(&current_node) {
                            min_path_length = num::integer::lcm(min_path_length, path_length);
                        }
                    }

                    min_path_length
                },
            )
            .reduce(|| 1, num::integer::lcm);
        println!("P2: {min_path_length}");
    }
}
