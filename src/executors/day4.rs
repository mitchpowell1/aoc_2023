use super::Executor;
use std::fmt::Write;

use nom::{
    bytes::complete::tag,
    bytes::complete::take_until,
    character::complete as cc,
    combinator::iterator,
    multi::fold_many1,
    sequence::{delimited, preceded, terminated},
    IResult,
};

#[derive(Debug)]
struct Card {
    num_winning: u32,
    copies: u32,
}

impl Card {
    fn parse(input: &str) -> IResult<&str, Card> {
        let (input, _) = terminated(take_until(":"), tag(":"))(input)?;

        let mut winning_numbers = [0u8; 10];
        let mut winning_numbers_iter = iterator(input, delimited(cc::space0, cc::u8, cc::space0));
        for (i, num) in winning_numbers_iter.enumerate() {
            winning_numbers[i] = num;
        }
        let res: IResult<_, _> = winning_numbers_iter.finish();
        let (input, _) = res.unwrap();

        let (input, num_winning) = preceded(
            tag("|"),
            fold_many1(
                preceded(cc::space1, cc::u8),
                || 0,
                |acc, val| {
                    if winning_numbers.contains(&val) {
                        acc + 1
                    } else {
                        acc
                    }
                },
            ),
        )(input)?;

        Ok((
            input,
            Card {
                num_winning,
                copies: 1,
            },
        ))
    }
}

#[derive(Default)]
pub struct Day4 {
    cards: Vec<Card>,
}

impl Executor for Day4 {
    fn parse(&mut self, input: String) {
        self.cards = input.lines().map(|l| Card::parse(l).unwrap().1).collect();
    }

    fn part_one(&mut self, output_buffer: &mut dyn Write) {
        let total: usize = self
            .cards
            .iter()
            .map(|Card { num_winning, .. }| match num_winning {
                0 => 0,
                _ => 1 << (num_winning - 1),
            })
            .sum();

        _ = write!(output_buffer, "P1: {total}");
    }

    fn part_two(&mut self, output_buffer: &mut dyn Write) {
        let mut total = 0;
        for i in 0..self.cards.len() {
            let Card {
                num_winning,
                copies,
            } = &self.cards[i];
            let additional_copies = *copies;
            let num_winning = *num_winning as usize;
            total += copies;
            for Card { copies, .. } in self.cards[(i + 1)..(i + 1) + num_winning].as_mut() {
                *copies += additional_copies;
            }
        }

        _ = write!(output_buffer, "P2: {total}");
    }
}
