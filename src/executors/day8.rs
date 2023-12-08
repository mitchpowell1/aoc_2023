use rustc_hash::FxHashMap;

use super::Executor;
use std::fmt::Write;

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
        sections
            .next()
            .unwrap()
            .chars()
            .for_each(|c| self.directions.push(Direction::from_char(c)));

        let nodes = sections.next().unwrap();
        let mut node_vec = vec![];
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
                Some(b'A') => self.a_nodes.push(i),
                Some(b'Z') => self.z_nodes.push(i),
                _ => {}
            }

            node_indexes.insert(node, i);
            node_vec.push((left, right));
        }
        node_vec.iter().for_each(|(l, r)| {
            self.map
                .push((*node_indexes.get(l).unwrap(), *node_indexes.get(r).unwrap()))
        });

        self.start = start;
        self.end = end;
    }

    fn part_one(&mut self, output_buffer: &mut dyn Write) {
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
        _ = write!(output_buffer, "P1: {num_steps}");
    }

    fn part_two(&mut self, output_buffer: &mut dyn Write) {
        let Day8 {
            directions,
            map,
            a_nodes,
            z_nodes,
            ..
        } = self;
        let min_path_length: u64 = a_nodes
            .iter()
            .map(|node| {
                let mut current_node = *node;
                let mut path_length = 0u64;
                let mut direction_index = 1;
                loop {
                    path_length += 1;

                    current_node = match directions[direction_index] {
                        Direction::Left => map[current_node].0,
                        Direction::Right => map[current_node].1,
                    };

                    if z_nodes.contains(&current_node) {
                        return path_length;
                    }

                    direction_index = (direction_index + 1) % directions.len();
                }
            })
            .product();
        _ = write!(output_buffer, "P2: {min_path_length}");
    }
}
