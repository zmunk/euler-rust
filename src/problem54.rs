use std::collections::HashMap;
use std::fs;
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Suit {
    D,
    S,
    C,
    H,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Card {
    rank: u8,
    suit: Suit,
}

impl Card {
    fn from(raw: &str) -> Card {
        let s = &raw[..1];
        let rank = match s.parse::<u8>() {
            Ok(i) => i,
            _ => match s {
                "T" => 10,
                "J" => 11,
                "Q" => 12,
                "K" => 13,
                "A" => 14,
                _ => panic!("invalid rank {:?}", s),
            },
        };
        let suit = match &raw[1..] {
            "D" => Suit::D,
            "S" => Suit::S,
            "C" => Suit::C,
            "H" => Suit::H,
            _ => panic!("invalid suit {:?}", &raw[1..]),
        };
        Card { rank, suit }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.rank.partial_cmp(&other.rank)
    }
}

fn is_flush(cards: &Vec<Card>) -> bool {
    let mut suit: Option<Suit> = None;
    for card in cards {
        if suit.is_some() && suit != Some(card.suit) {
            return false
        }
        suit = Some(card.suit);
    }
    return true
}

fn is_straight(sorted_ranks: &Vec<u8>) -> bool {
    let mut prev = 0;
    for &rank in sorted_ranks {
        if prev != 0 && rank != prev - 1 {
            return false
        }
        prev = rank;
    }
    true
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    counts_desc: Vec<(u8, u8)>,
    is_flush: bool,
    is_straight: bool,
}

impl Hand {
    fn from(raw: &str) -> Hand {
        let mut cards = vec![];
        for c in raw.split(" ") {
            cards.push(Card::from(c));
        }
        let mut rank_counts = HashMap::new();
        let mut sorted_ranks = vec![];
        for card in &cards {
            insert_desc(&card.rank, &mut sorted_ranks);
            let entry = rank_counts.entry(card.rank).or_insert(0_u8);
            *entry += 1;
        }
        let mut counts_desc: Vec<(u8, u8)> = vec![];
        for (&rank, &count) in rank_counts.iter() {
            insert_desc(&(count, rank), &mut counts_desc);
        }
        let is_flush = is_flush(&cards);
        let is_straight = is_straight(&sorted_ranks);
        Hand { cards, counts_desc, is_flush, is_straight }
    }

    fn cmp_with_func(&self, other: &Hand, func: &dyn Fn(&Hand) -> bool) -> Option<Ordering> {
        if func(self) {
            if func(other) {
                match self.counts_desc.partial_cmp(&other.counts_desc) {
                    Some(Ordering::Equal) => None,
                    cmp => cmp,
                }
            } else {
                Some(Ordering::Greater)
            }
        } else if func(other) {
            Some(Ordering::Less) 
        } else {
            None
        }
    }

    fn is_royal_flush(&self) -> bool {
        self.is_flush && self.counts_desc.iter().map(|(count, _)| *count).collect::<Vec<u8>>()
            == [10, 11, 12, 13, 14]
    }

    fn is_straight_flush(&self) -> bool {
        self.is_flush && self.is_straight
    }

    fn has_four_of_kind(&self) -> bool {
        self.counts_desc[0].0 == 4
    }

    fn is_full_house(&self) -> bool {
        self.counts_desc[0].0 == 3 && self.counts_desc[1].0 == 2
    }

    fn has_three_of_kind(&self) -> bool {
        self.counts_desc[0].0 == 3
    }

    fn has_one_pair(&self) -> bool {
        self.counts_desc[0].0 == 2
    }

    fn has_two_pairs(&self) -> bool {
        self.counts_desc[0].0 == 2 && self.counts_desc[1].0 == 2
    }
}

/// insert preserving descending order
fn insert_desc<T>(value: &T, arr: &mut Vec<T>) where T: PartialOrd + Copy {
    let (mut left, mut right) = (0, arr.len());
    let mut mid;
    loop {
        mid = (left + right) / 2;
        if left == right {
            break;
        } else if value < &arr[mid] {
            left = mid + 1;
        } else if value > &arr[mid] {
            right = mid;
        } else {
            break;
        }
    }
    arr.insert(mid, *value);
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        for (a, b) in self.cards.iter().zip(other.cards.iter()) {
            if a != b {
                return false
            }
        }
        true
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        for func in [
            |a: &Hand| a.is_royal_flush(),
            |a: &Hand| a.is_straight_flush(),
            |a: &Hand| a.has_four_of_kind(),
            |a: &Hand| a.is_full_house(),
            |a: &Hand| a.is_flush,
            |a: &Hand| a.is_straight,
            |a: &Hand| a.has_three_of_kind(),
            |a: &Hand| a.has_two_pairs(),
            |a: &Hand| a.has_one_pair(),
        ] {
            match self.cmp_with_func(other, &func) {
                None => (),
                cmp => return cmp,
            }
        }
        self.counts_desc.partial_cmp(&other.counts_desc)
    }
}

pub fn main() -> i32 {
    let contents = fs::read_to_string("data/problem54.txt")
        .expect("unable to read file");
    let rows: Vec<&str> = contents.trim().split("\n").collect();
    let mut count = 0;
    for row in rows {
        let player1 = Hand::from(&row[..14]);
        let player2 = Hand::from(&row[15..]);
        if player1 > player2 {
            count += 1;
        }
    }
    count
}

#[test]
fn test() {
    assert!(Hand::from("5H 5C 6S 7S KD") < Hand::from("2C 3S 8S 8D TD"));
    assert!(Hand::from("5D 8C 9S JS AC") > Hand::from("2C 5C 7D 8S QH"));
    assert!(Hand::from("2D 9C AS AH AC") < Hand::from("3D 6D 7D TD QD"));
    assert!(Hand::from("4D 6S 9H QH QC") > Hand::from("3D 6D 7H QD QS"));
    assert!(Hand::from("2H 2D 4C 4D 4S") > Hand::from("3C 3D 3S 9S 9D"));
}
