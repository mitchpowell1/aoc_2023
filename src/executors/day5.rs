use super::Executor;
use crate::utils::partitioned_by::*;

use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::space1;
use nom::character::complete::u64 as ccu64;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;

use std::fmt::Write;
use std::ops::Range;

#[derive(Debug)]
struct Map {
    destination_start: u64,
    source_start: u64,
    size: u64,
}

impl Map {
    fn map_value(&self, value: u64) -> Option<u64> {
        let range = Range {
            start: self.source_start,
            end: self.source_start + self.size,
        };
        let offset = self.destination_start as i64 - self.source_start as i64;
        if range.contains(&value) {
            let out = (value as i64 + offset) as u64;
            return Some(out);
        }

        None
    }

    fn map_range(&self, range: Range<u64>) -> Partitions<Range<u64>> {
        let map_range = self.source_start..self.source_start + self.size;
        let offset = self.destination_start as i64 - self.source_start as i64;
        let mut partitions = range.partitioned_by(&map_range);
        if let Some(m) = &mut partitions.middle {
            m.start = (m.start as i64 + offset) as u64;
            m.end = (m.end as i64 + offset) as u64;
        }
        partitions
    }

    fn parse(input: &str) -> IResult<&str, Map> {
        let (input, (destination_start, source_start, size)) =
            tuple((terminated(ccu64, space1), terminated(ccu64, space1), ccu64))(input)?;

        Ok((
            input,
            Map {
                destination_start,
                source_start,
                size,
            },
        ))
    }
}

#[derive(Default, Debug)]
pub struct Day5 {
    seeds: Vec<u64>,
    map_cascade: Vec<Vec<Map>>,
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
    let (i, seeds) = preceded(tag("seeds: "), separated_list1(space1, ccu64))(input)?;
    Ok((i, seeds))
}

fn parse_map_section(input: &str) -> IResult<&str, Vec<Map>> {
    let (input, _title) = terminated(take_until(" map:\n"), tag(" map:\n"))(input)?;
    let (input, map_section) = separated_list1(line_ending, Map::parse)(input)?;
    Ok((input, map_section))
}

impl Executor for Day5 {
    fn parse(&mut self, input: String) {
        let mut sections = input.split("\n\n");
        let seeds_section = sections.next().unwrap();
        let seeds = parse_seeds(seeds_section).unwrap().1;

        let mut map_cascade = vec![];
        for section in sections {
            let section_map = parse_map_section(section).unwrap().1;
            map_cascade.push(section_map);
        }

        self.seeds = seeds;
        self.map_cascade = map_cascade;
    }

    fn part_one(&mut self, output_buffer: &mut dyn Write) {
        let mut min_location = u64::MAX;
        for seed in &self.seeds {
            let mut v = *seed;
            for map_list in &self.map_cascade {
                for map in map_list {
                    if let Some(d) = map.map_value(v) {
                        v = d;
                        break;
                    }
                }
            }

            min_location = std::cmp::min(min_location, v);
        }

        _ = write!(output_buffer, "P1: {min_location}");
    }

    fn part_two(&mut self, output_buffer: &mut dyn Write) {
        let mut inputs: Vec<_> = self
            .seeds
            .chunks_exact(2)
            .map(|r| r[0]..r[0] + r[1])
            .collect();
        let mut outputs = vec![];
        let mut buffer = vec![];
        for map_vec in &self.map_cascade {
            for map in map_vec {
                for range in &mut inputs.drain(..) {
                    let Partitions {
                        lower,
                        middle,
                        upper,
                    } = map.map_range(range);
                    buffer.extend(lower.into_iter());
                    buffer.extend(upper.into_iter());
                    outputs.extend(middle.into_iter());
                }
                std::mem::swap(&mut inputs, &mut buffer);
            }
            outputs.append(&mut inputs);
            std::mem::swap(&mut outputs, &mut inputs);
        }
        std::mem::swap(&mut outputs, &mut inputs);

        let min_location = outputs
            .iter()
            .fold(u64::MAX, |acc, rng| std::cmp::min(acc, rng.start));
        _ = write!(output_buffer, "P2: {min_location}");
    }
}
