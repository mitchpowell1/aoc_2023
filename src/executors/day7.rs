use super::Executor;

use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::mem::MaybeUninit;

#[derive(Debug, PartialEq, Copy, Clone, Eq)]
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

    fn evaluating_jacks_as_jokers(self) -> Self {
        if self == Card::Jack {
            Card::Joker
        } else {
            self
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_order_key().cmp(&other.get_order_key())
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Eq, Ord)]
enum Hand {
    HighCard([Card; 5]),
    Pair([Card; 5]),
    TwoPair([Card; 5]),
    ThreeOfKind([Card; 5]),
    FullHouse([Card; 5]),
    FourOfKind([Card; 5]),
    FiveOfKind([Card; 5]),
}

impl Hand {
    fn with_jacks_as_jokers(&self) -> Self {
        use Hand::*;
        let mut cards = match self {
            HighCard(c) | Pair(c) | TwoPair(c) | ThreeOfKind(c) | FullHouse(c) | FourOfKind(c)
            | FiveOfKind(c) => *c,
        };

        for card in &mut cards {
            *card = Card::evaluating_jacks_as_jokers(*card);
        }

        Hand::from_cards(cards)
    }

    fn from_cards(cards: [Card; 5]) -> Hand {
        let mut counts = [0; 14];
        for card in &cards {
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
            [.., 5] => Hand::FiveOfKind(cards),
            [.., 4] => Hand::FourOfKind(cards),
            [.., 2, 3] => Hand::FullHouse(cards),
            [.., 3] => Hand::ThreeOfKind(cards),
            [.., 2, 2] => Hand::TwoPair(cards),
            [.., 2] => Hand::Pair(cards),
            _ => Hand::HighCard(cards),
        }
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
        let _input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let mut hands = vec![];
        for line in input.lines() {
            let mut parts = line.split(' ');
            let raw_hand = parts.next().unwrap();
            let raw_bid = parts.next().unwrap();

            let mut hand: [MaybeUninit<Card>; 5] =
                unsafe { std::mem::MaybeUninit::uninit().assume_init() };

            for (i, c) in raw_hand.chars().enumerate() {
                hand[i].write(Card::from_char(c));
            }

            let hand = unsafe { std::mem::transmute::<_, [Card; 5]>(hand) };

            let mut bid = 0u32;
            for c in raw_bid.chars() {
                bid *= 10;
                bid += c.to_digit(10).unwrap();
            }
            hands.push((Hand::from_cards(hand), bid));
        }
        self.hands = hands;
    }

    fn part_one(&mut self) {
        self.hands.sort_by(|(h1, _b1), (h2, _b2)| h1.cmp(h2));

        println!("P1: {}", self.compute_total_winnings());
    }

    fn part_two(&mut self) {
        self.hands
            .iter_mut()
            .for_each(|(hand, _)| *hand = hand.with_jacks_as_jokers());

        self.hands.sort_by(|(h1, _), (h2, _)| h1.cmp(h2));
        println!("P2: {}", self.compute_total_winnings());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn five_of_a_kind_classification() {
        let cards = [Card::Ace; 5];
        let hand = Hand::from_cards(cards.clone());
        assert_eq!(hand, Hand::FiveOfKind(cards));
    }

    #[test]
    fn five_of_a_kind_classification_joker() {
        let mut cards = [Card::Ace; 5];
        cards[0] = Card::Joker;
        let hand = Hand::from_cards(cards.clone());
        assert_eq!(hand, Hand::FiveOfKind(cards));
    }

    #[test]
    fn four_of_a_kind_classification() {
        let mut cards = [Card::Ace; 5];
        cards[0] = Card::Number(2);
        let hand = Hand::from_cards(cards.clone());
        assert_eq!(hand, Hand::FourOfKind(cards));
    }

    #[test]
    fn four_of_a_kind_classification_joker() {
        let mut cards = [Card::Ace; 5];
        cards[0] = Card::Number(2);
        cards[1] = Card::Joker;
        let hand = Hand::from_cards(cards.clone());
        assert_eq!(hand, Hand::FourOfKind(cards));
    }

    #[test]
    fn full_house_classification() {
        let mut cards = [Card::Ace; 5];
        cards[0] = Card::Number(2);
        cards[1] = Card::Number(2);
        let hand = Hand::from_cards(cards.clone());
        assert_eq!(hand, Hand::FullHouse(cards));
    }

    #[test]
    fn three_of_a_kind_classification() {
        let mut cards = [Card::Ace; 5];
        cards[0] = Card::Number(2);
        cards[1] = Card::Number(3);
        let hand = Hand::from_cards(cards.clone());
        assert_eq!(hand, Hand::ThreeOfKind(cards));
    }

    #[test]
    fn two_pair_classification() {
        let mut cards = [Card::Ace; 5];
        cards[0] = Card::Number(2);
        cards[1] = Card::Number(2);
        cards[2] = Card::Jack;
        let hand = Hand::from_cards(cards.clone());
        assert_eq!(hand, Hand::TwoPair(cards));
    }

    #[test]
    fn pair_classification() {
        let mut cards = [Card::Ace; 5];
        cards[0] = Card::Number(2);
        cards[1] = Card::Number(3);
        cards[2] = Card::Jack;
        let hand = Hand::from_cards(cards.clone());
        assert_eq!(hand, Hand::Pair(cards));
    }

    #[test]
    fn high_card_classification() {
        let cards = [
            Card::Number(2),
            Card::Number(3),
            Card::Number(4),
            Card::Number(5),
            Card::Number(6),
        ];
        let hand = Hand::from_cards(cards.clone());
        assert_eq!(hand, Hand::HighCard(cards));
    }

    #[test]
    fn all_jokers_classification() {
        let cards = [Card::Joker; 5];
        let hand = Hand::from_cards(cards.clone());
        assert_eq!(hand, Hand::FiveOfKind(cards));
    }

    #[test]
    fn sanity_check_card_ordering() {
        assert!(Card::Number(10) > Card::Number(3));
        assert!(Card::Ace > Card::Number(10));
        assert!(Card::King > Card::Queen);
        assert!(Card::Jack < Card::Queen);
        assert!(Card::Jack == Card::Jack);
        assert!(Card::Number(8) == Card::Number(8));
        assert!(Card::Joker < Card::Number(2));
    }

    #[test]
    fn sanity_check_hand_ordering_jokers() {
        let hand1 = Hand::from_cards([
            Card::Joker,
            Card::Number(2),
            Card::Number(3),
            Card::Number(4),
            Card::Number(2),
        ]);
        let hand2 = Hand::from_cards([
            Card::Number(2),
            Card::Number(2),
            Card::Number(3),
            Card::Number(4),
            Card::Number(2),
        ]);
        assert!(matches!(hand2, Hand::ThreeOfKind(_)));
        assert!(matches!(hand1, Hand::ThreeOfKind(_)));
        assert!(hand1 < hand2);
    }

    #[test]
    fn sanity_check_hand_ordering_different_classifications() {
        let hand1 = Hand::from_cards([Card::Ace, Card::Ace, Card::King, Card::King, Card::King]);
        let hand2 = Hand::from_cards([Card::Ace, Card::Ace, Card::King, Card::King, Card::Jack]);
        assert!(hand1 > hand2);
    }

    #[test]
    fn sanity_check_hand_ordering_same_classifications() {
        // Full house Kings over Aces, Leading King
        let hand1 = Hand::from_cards([Card::King, Card::King, Card::Ace, Card::Ace, Card::Ace]);
        // Full house Aces over Kings, Leading card Ace
        let hand2 = Hand::from_cards([Card::Ace, Card::Ace, Card::King, Card::King, Card::King]);
        assert!(hand2 > hand1);
    }

    #[test]
    fn sanity_check_hand_ordering_identical() {
        // Full house Kings over Aces, Leading King
        let hand1 = Hand::from_cards([Card::King, Card::King, Card::Ace, Card::Ace, Card::Ace]);
        // Full house Aces over Kings, Leading card Ace
        let hand2 = Hand::from_cards([Card::King, Card::King, Card::Ace, Card::Ace, Card::Ace]);
        assert!(hand2 == hand1);
    }

    #[test]
    fn sanity_check_card_reevaluation() {
        assert_eq!(Card::Jack.evaluating_jacks_as_jokers(), Card::Joker);
        assert_eq!(Card::King.evaluating_jacks_as_jokers(), Card::King);
        assert_eq!(
            Card::Number(10).evaluating_jacks_as_jokers(),
            Card::Number(10)
        );
    }
}
