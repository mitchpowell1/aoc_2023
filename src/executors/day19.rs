use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{one_of, u64},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};
use rustc_hash::FxHashMap;

use super::Executor;
use std::{fmt::Write, ops::RangeInclusive};

#[derive(Debug, Clone, Copy)]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Copy, Clone)]
enum Criteria {
    LessThan(u64),
    GreaterThan(u64),
}

impl Criteria {
    fn is_satisfied_by(&self, value: u64) -> bool {
        match self {
            Criteria::LessThan(comparator) => value < *comparator,
            Criteria::GreaterThan(comparator) => value > *comparator,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Verdict {
    Accept,
    Reject,
    RouteTo(usize),
}

#[derive(Debug)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

#[derive(Debug, Clone)]
struct RangePart {
    ratings: [RangeInclusive<u16>; 4],
}

impl RangePart {
    fn count(&self) -> u64 {
        self.ratings.iter().map(|r| r.len() as u64).product()
    }
}

#[derive(Debug)]
struct Workflow {
    workstreams: [Option<(Category, Criteria, Verdict)>; 4],
    workflow_verdict: Verdict,
}

impl Workflow {
    fn evaluate_part(&self, p: &Part) -> Verdict {
        for i in 0..4 {
            if let Some((category, criteria, verdict)) = &self.workstreams[i] {
                let rating = match category {
                    Category::X => p.x,
                    Category::M => p.m,
                    Category::A => p.a,
                    Category::S => p.s,
                };
                if criteria.is_satisfied_by(rating) {
                    return *verdict;
                }
            }
        }
        self.workflow_verdict
    }
}

fn parse_workstream<'a>(
    input: &'a str,
    labels: &'a FxHashMap<&'a str, usize>,
) -> IResult<&'a str, (char, Criteria, Verdict)> {
    // X -> M -> A -> S -> Final
    let (input, ((ws_category, crit_symbol, comparator), destination)) = terminated(
        separated_pair(
            tuple((one_of("xmas"), one_of("><"), u64)),
            tag(":"),
            take_until(","),
        ),
        tag(","),
    )(input)?;
    let verdict = match destination {
        "A" => Verdict::Accept,
        "R" => Verdict::Reject,
        other => Verdict::RouteTo(labels[other]),
    };
    let criteria = match crit_symbol {
        '>' => Criteria::GreaterThan(comparator),
        '<' => Criteria::LessThan(comparator),
        _ => unreachable!(),
    };
    Ok((input, (ws_category, criteria, verdict)))
}

fn parse_workflow<'a>(
    input: &'a str,
    label_map: &'a FxHashMap<&'a str, usize>,
) -> IResult<&'a str, Workflow> {
    let (input, mut ws) = preceded(
        take_until("{"),
        delimited(tag("{"), take_until("}"), tag("}")),
    )(input)?;

    let mut workstreams = [None; 4];
    let mut idx = 0;
    while let Ok((i, (category, criteria, verdict))) = parse_workstream(ws, label_map) {
        let category = match category {
            'x' => Category::X,
            'm' => Category::M,
            'a' => Category::A,
            's' => Category::S,
            _ => unreachable!(),
        };
        workstreams[idx] = Some((category, criteria, verdict));
        idx += 1;
        ws = i;
    }

    let workflow_verdict = match ws {
        "A" => Verdict::Accept,
        "R" => Verdict::Reject,
        other => Verdict::RouteTo(label_map[other]),
    };

    let workflow = Workflow {
        workstreams,
        workflow_verdict,
    };

    Ok((input, workflow))
}

fn parse_part(input: &str) -> IResult<&str, Part> {
    let (input, (x, m, a, s)) = tuple((
        delimited(tag("{x="), u64, tag(",")),
        delimited(tag("m="), u64, tag(",")),
        delimited(tag("a="), u64, tag(",")),
        delimited(tag("s="), u64, tag("}")),
    ))(input)?;

    Ok((input, Part { x, m, a, s }))
}

#[derive(Default)]
pub struct Day19 {
    start: usize,
    workflows: Vec<Workflow>,
    parts: Vec<Part>,
}

impl Day19 {
    fn evaluate_part_range(&self, range_part: RangePart) -> u64 {
        fn helper(range_part: RangePart, workflow_idx: usize, workflows: &[Workflow]) -> u64 {
            if range_part.count() == 0 {
                return 0;
            }
            let workflow = &workflows[workflow_idx];
            let mut out = 0;
            let mut unmatching_parts = range_part.clone();
            for i in 0..4 {
                if let Some((category, criteria, verdict)) = &workflow.workstreams[i] {
                    let rating_index = match category {
                        Category::X => 0,
                        Category::M => 1,
                        Category::A => 2,
                        Category::S => 3,
                    };

                    let mut matching_parts = unmatching_parts.clone();
                    let matching_rating = &mut matching_parts.ratings[rating_index];
                    let unmatching_rating = &mut unmatching_parts.ratings[rating_index];
                    match criteria {
                        Criteria::GreaterThan(v) => {
                            *matching_rating =
                                std::cmp::max((v + 1) as u16, *matching_rating.start())
                                    ..=*matching_rating.end();
                            *unmatching_rating = *unmatching_rating.start()
                                ..=std::cmp::min(*unmatching_rating.end(), *v as u16);
                        }
                        Criteria::LessThan(v) => {
                            *matching_rating = *matching_rating.start()
                                ..=std::cmp::min(*matching_rating.end(), (v - 1) as u16);
                            *unmatching_rating =
                                std::cmp::max(*v as u16, *unmatching_rating.start())
                                    ..=*unmatching_rating.end();
                        }
                    };
                    out += match verdict {
                        Verdict::Accept => matching_parts.count(),
                        Verdict::RouteTo(idx) => helper(matching_parts, *idx, workflows),
                        _ => 0,
                    }
                }
            }
            out += match workflow.workflow_verdict {
                Verdict::Accept => unmatching_parts.count(),
                Verdict::RouteTo(idx) => helper(unmatching_parts, idx, workflows),
                _ => 0,
            };
            out
        }
        helper(range_part, self.start, &self.workflows)
    }
}

impl Executor for Day19 {
    fn parse(&mut self, input: String) {
        let mut groups = input.split("\n\n");
        let rules = groups.next().unwrap();
        let parts = groups.next().unwrap();
        let mut label_hash = FxHashMap::default();
        for (i, line) in rules.lines().enumerate() {
            let label = line.split('{').next().unwrap();
            label_hash.insert(label, i);
        }
        for line in rules.lines() {
            self.workflows
                .push(parse_workflow(line, &label_hash).unwrap().1);
        }
        for line in parts.lines() {
            self.parts.push(parse_part(line).unwrap().1);
        }
        self.start = label_hash["in"];
    }

    fn part_one(&mut self, output_buffer: &mut dyn Write) {
        let mut accepted_total = 0u64;
        for part in &self.parts {
            let mut workflow = self.start;
            let v = loop {
                match self.workflows[workflow].evaluate_part(part) {
                    Verdict::RouteTo(v) => workflow = v,
                    other => break other,
                }
            };
            if let Verdict::Accept = v {
                accepted_total += part.x + part.m + part.a + part.s
            }
        }
        _ = write!(output_buffer, "P1: {accepted_total}");
    }

    fn part_two(&mut self, output_buffer: &mut dyn Write) {
        let range_part = RangePart {
            ratings: std::array::from_fn(|_| 1..=4000),
        };
        let out = self.evaluate_part_range(range_part);
        _ = write!(output_buffer, "P2: {out}");
    }
}
