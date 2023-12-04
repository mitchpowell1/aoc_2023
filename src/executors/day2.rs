use super::Executor;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete as cc,
    combinator::opt,
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated},
    Finish, IResult,
};

#[derive(Default)]
pub struct Day2 {
    games: Vec<Game>,
}

#[derive(Debug)]
struct Game {
    id: i32,
    samples: Vec<Sample>,
}

impl Game {
    fn parse(input: &str) -> IResult<&str, Game> {
        let (input, id) = delimited(tag("Game "), cc::i32, tag(": "))(input)?;
        let (input, samples) = separated_list1(tag("; "), Sample::parse)(input)?;
        Ok((input, Game { id, samples }))
    }
}

#[derive(Debug, Default)]
struct Sample(i32, i32, i32);

impl Sample {
    fn parse(input: &str) -> IResult<&str, Sample> {
        let mut input = input;
        let mut out = Sample(0, 0, 0);
        while !input.starts_with(';') && !input.is_empty() {
            let (s, (count, color)) = terminated(
                separated_pair(
                    cc::i32,
                    cc::space1,
                    alt((tag("green"), tag("blue"), tag("red"))),
                ),
                opt(tag(", ")),
            )(input)?;

            match color {
                "red" => out.0 = count,
                "green" => out.1 = count,
                "blue" => out.2 = count,
                _ => {}
            }

            input = s
        }

        Ok((input, out))
    }
}

impl Executor for Day2 {
    fn parse(&mut self, input: String) {
        self.games = input
            .lines()
            .map(|g| Game::parse(g).finish().unwrap().1)
            .collect();
    }

    fn part_one(&mut self) {
        let mut p1_sum = 0;
        let mut p2_sum = 0;
        for game in &self.games {
            let mut max_blue = 0;
            let mut max_green = 0;
            let mut max_red = 0;

            for &Sample(r, g, b) in &game.samples {
                max_blue = std::cmp::max(max_blue, r);
                max_red = std::cmp::max(max_red, g);
                max_green = std::cmp::max(max_green, b);
            }
            if max_red <= 12 && max_green <= 13 && max_blue <= 14 {
                p1_sum += game.id;
            }
            p2_sum += max_red * max_blue * max_green
        }
        println!("P1: {p1_sum}");
        println!("P2: {p2_sum}");
    }

    fn part_two(&mut self) {}
}
