use super::Executor;

use std::cmp::PartialOrd;
use std::mem::MaybeUninit;

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone, Eq, Ord)]
enum Card {
    Joker,
    Number(u8),
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from_char(c: char) -> Card {
        match c {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Jack,
            'T' => Card::Number(10),
            _ => Card::Number(c.to_digit(10).unwrap() as u8),
        }
    }

    fn get_order_key(&self) -> u8 {
        match self {
            Card::Ace => 13,
            Card::King => 12,
            Card::Queen => 11,
            Card::Jack => 10,
            Card::Number(n) => *n - 1,
            Card::Joker => 0,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Eq, Ord)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

impl HandType {
    fn from_cards(cards: &[Card; 5]) -> HandType {
        let mut counts = [0; 14];
        for card in cards {
            counts[card.get_order_key() as usize] += 1;
        }
        let jokers = counts[0];
        let max_index = counts[1..]
            .iter()
            .enumerate()
            .fold((1, 0), |(max_i, max_count), (i, count)| {
                if *count >= max_count {
                    (i, *count)
                } else {
                    (max_i, max_count)
                }
            })
            .0;

        counts[max_index] += jokers;
        counts[1..].sort();
        match counts[1..] {
            [.., 5] => HandType::FiveOfKind,
            [.., 4] => HandType::FourOfKind,
            [.., 2, 3] => HandType::FullHouse,
            [.., 3] => HandType::ThreeOfKind,
            [.., 2, 2] => HandType::TwoPair,
            [.., 2] => HandType::Pair,
            _ => HandType::HighCard,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Eq, Ord)]
struct Hand(HandType, [Card; 5]);

impl Hand {
    fn with_jacks_as_jokers(&mut self) {
        let Hand(hand_type, cards) = self;
        cards
            .iter_mut()
            .filter(|c| matches!(c, Card::Jack))
            .for_each(|c| *c = Card::Joker);
        *hand_type = HandType::from_cards(&cards)
    }
}

#[derive(Default, Debug)]
pub struct Day7 {
    hands: Vec<(Hand, u32)>,
}

impl Day7 {
    fn compute_total_winnings(&self) -> u32 {
        self.hands
            .iter()
            .enumerate()
            .fold(0, |acc, (i, (_h, bet))| acc + ((i as u32 + 1) * bet))
    }
}

impl Executor for Day7 {
    fn parse(&mut self, input: String) {
        for line in input.lines() {
            let mut parts = line.split(' ');
            let raw_hand = parts.next().unwrap();
            let raw_bid = parts.next().unwrap();

            let mut cards: [MaybeUninit<Card>; 5] =
                unsafe { std::mem::MaybeUninit::uninit().assume_init() };

            for (i, c) in raw_hand.chars().enumerate() {
                cards[i].write(Card::from_char(c));
            }

            let cards = unsafe { std::mem::transmute::<_, [Card; 5]>(cards) };

            let mut bid = 0u32;
            for c in raw_bid.chars() {
                bid *= 10;
                bid += c.to_digit(10).unwrap();
            }
            self.hands
                .push((Hand(HandType::from_cards(&cards), cards), bid));
        }
    }

    fn part_one(&mut self) {
        self.hands.sort_unstable();

        println!("P1: {}", self.compute_total_winnings());
    }

    fn part_two(&mut self) {
        self.hands
            .iter_mut()
            .for_each(|(hand, _)| hand.with_jacks_as_jokers());

        self.hands.sort_unstable();
        println!("P2: {}", self.compute_total_winnings());
    }
}
