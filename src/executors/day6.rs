use nom::{
    bytes::complete::tag,
    character::complete::{newline, space1, u64},
    combinator::opt,
    multi::fold_many1,
    sequence::{delimited, preceded},
    IResult,
};

use super::Executor;

#[derive(Default, Debug)]
pub struct Day6 {
    times: [Option<u64>; 4],
    distances: [Option<u64>; 4],
}

fn parse_vals<'a>(
    input: &'a str,
    prefix: &'a str,
    slice: &'a mut [Option<u64>],
) -> IResult<&'a str, ()> {
    let (input, _) = delimited(
        tag(prefix),
        fold_many1(
            preceded(space1, u64),
            || 0,
            |acc, val| {
                slice[acc] = Some(val);
                acc + 1
            },
        ),
        opt(newline),
    )(input)?;
    Ok((input, ()))
}

fn get_num_winning_charge_times(available_time: u64, record_distance: u64) -> u64 {
    let mut lower_bound = 0;
    let mut upper_bound = available_time / 2;

    let get_distance = |total_time, hold_time| (total_time - hold_time) * hold_time;

    let lower = loop {
        if lower_bound >= upper_bound {
            break lower_bound;
        }

        let midpoint = (upper_bound + lower_bound) / 2;
        if get_distance(available_time, midpoint) <= record_distance {
            lower_bound = midpoint + 1;
        } else {
            upper_bound = midpoint;
        }
    };

    upper_bound = available_time;
    let upper = loop {
        if lower_bound >= upper_bound {
            break upper_bound;
        }
        let midpoint = (upper_bound + lower_bound) / 2;
        if get_distance(available_time, midpoint) > record_distance {
            lower_bound = midpoint + 1;
        } else {
            upper_bound = midpoint;
        }
    };

    upper - lower
}

impl Executor for Day6 {
    fn parse(&mut self, input: String) {
        let (input, _) = parse_vals(&input, "Time:", &mut self.times).unwrap();
        let _ = parse_vals(input, "Distance:", &mut self.distances).unwrap();
    }

    fn part_one(&mut self) {
        let total = self
            .times
            .iter()
            .flatten()
            .zip(self.distances.iter().flatten())
            .fold(1, |acc, (time, distance)| {
                acc * get_num_winning_charge_times(*time, *distance)
            });

        println!("P1: {total}");
    }

    fn part_two(&mut self) {
        let time = self
            .times
            .iter()
            .flatten()
            .fold(0, |acc, val| acc * 10u64.pow(val.ilog10() + 1) + val);

        let distance = self
            .distances
            .iter()
            .flatten()
            .fold(0, |acc, val| acc * 10u64.pow(val.ilog10() + 1) + val);
        let num_record_breaking_runs = get_num_winning_charge_times(time, distance);
        println!("P2: {num_record_breaking_runs}");
    }
}
