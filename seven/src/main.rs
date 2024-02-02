use std::{
  cmp::Ordering,
  collections::{HashMap, HashSet},
};

use aoclib::parse_input_lines;
use nom::IResult;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Card {
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Nine,
  Ten,
  Jack,
  Queen,
  King,
  Ace,
}

impl Card {
  fn from(text: char) -> Option<Card> {
    match text {
      '2' => Some(Card::Two),
      '3' => Some(Card::Three),
      '4' => Some(Card::Four),
      '5' => Some(Card::Five),
      '6' => Some(Card::Six),
      '7' => Some(Card::Seven),
      '8' => Some(Card::Eight),
      '9' => Some(Card::Nine),
      'T' => Some(Card::Ten),
      'J' => Some(Card::Jack),
      'Q' => Some(Card::Queen),
      'K' => Some(Card::King),
      'A' => Some(Card::Ace),
      _ => None,
    }
  }

  fn score(&self, part_two: bool) -> u32 {
    match self {
      Card::Two => 2,
      Card::Three => 3,
      Card::Four => 4,
      Card::Five => 5,
      Card::Six => 6,
      Card::Seven => 7,
      Card::Eight => 8,
      Card::Nine => 9,
      Card::Ten => 10,
      Card::Jack => {
        if part_two {
          1
        } else {
          11
        }
      }
      Card::Queen => 12,
      Card::King => 13,
      Card::Ace => 14,
    }
  }

  fn cmp(&self, other: &Card, part_two: bool) -> std::cmp::Ordering {
    self.score(part_two).cmp(&other.score(part_two))
  }
}

#[derive(Debug, Clone)]
struct Hand {
  cards: Vec<Card>,
}

impl Hand {
  /// Count the amount of jokers, for part two
  /// Don't collect joker cards into a unique list, because they're used to increase
  /// the amount of the other cards.
  /// For each unique card found in a hand, keep track how often it appears
  /// For example, with hand "JJAAK", the result is a map[2] = 2 entry which means
  /// the hand contains 2 pairs
  /// For part two, if there are no unique cards, the hand is a five of a kind Jack
  fn determine_type(&self, part_two: bool) -> HandType {
    let jack_count = self.cards.iter().filter(|&c| c == &Card::Jack).count() as u32;
    if jack_count == 5 {
      return HandType::FiveOfAKind;
    }
    let unique_cards: HashSet<&Card> = self
      .cards
      .iter()
      .filter(|&c| !part_two || c != &Card::Jack)
      .collect();
    let mut card_counts: HashMap<u32, u32> = HashMap::new();
    for card in unique_cards.iter() {
      let count = self.cards.iter().filter(|&c| c == *card).count() as u32;
      *card_counts.entry(count).or_insert(0) += 1;
    }
    if part_two {
      let max_same_card = *card_counts.keys().max().unwrap();
      if max_same_card == 5 - jack_count {
        return HandType::FiveOfAKind;
      }
      *card_counts.entry(max_same_card + jack_count).or_insert(0) += 1;
      if let Some(count) = card_counts.get_mut(&max_same_card) {
        *count -= 1;
        if *count == 0 {
          card_counts.remove(&max_same_card);
        }
      }
    }
    if card_counts.get(&5).is_some() {
      HandType::FiveOfAKind
    } else if card_counts.get(&4).is_some() {
      HandType::FourOfAKind
    } else if card_counts.get(&3).is_some() && card_counts.get(&2).is_some() {
      HandType::FullHouse
    } else if card_counts.get(&3).is_some() {
      HandType::ThreeOfAKind
    } else if card_counts.get(&2).map_or(0, |&v| v) >= 2 {
      HandType::TwoPair
    } else if card_counts.get(&2).is_some() {
      HandType::OnePair
    } else {
      HandType::HighCard
    }
  }
  fn from(text: &str) -> Option<Hand> {
    let mut cards = Vec::new();
    for card_char in text.chars() {
      let card = Card::from(card_char)?;
      cards.push(card);
    }
    Some(Hand { cards })
  }

  fn cmp(&self, other: &Hand, part_two: bool) -> std::cmp::Ordering {
    let own_type = self.determine_type(part_two);
    let other_type = other.determine_type(part_two);
    match own_type.cmp(&other_type) {
      Ordering::Equal => {
        for (own_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
          match own_card.cmp(other_card, part_two) {
            Ordering::Equal => continue,
            _ => {
              return own_card.cmp(other_card, part_two);
            }
          }
        }
        panic!("Hands are equal");
      }
      _ => own_type.cmp(&other_type),
    }
  }
}

#[derive(Debug)]
enum HandType {
  HighCard,
  OnePair,
  TwoPair,
  ThreeOfAKind,
  FullHouse,
  FourOfAKind,
  FiveOfAKind,
}

impl HandType {
  fn score(&self) -> u32 {
    match self {
      HandType::HighCard => 1,
      HandType::OnePair => 2,
      HandType::TwoPair => 3,
      HandType::ThreeOfAKind => 4,
      HandType::FullHouse => 5,
      HandType::FourOfAKind => 6,
      HandType::FiveOfAKind => 7,
    }
  }

  fn cmp(&self, other: &HandType) -> std::cmp::Ordering {
    self.score().cmp(&other.score())
  }
}

fn parse_line(input: &str) -> IResult<&str, (Hand, u32)> {
  let (remainder, (hand_text, _, score)) = nom::sequence::tuple((
    nom::character::complete::alphanumeric1,
    nom::character::complete::space1,
    nom::character::complete::digit1,
  ))(input)?;
  let hand = Hand::from(hand_text).unwrap();
  let score = score.parse::<u32>().unwrap();
  Ok((remainder, (hand, score)))
}

fn main() {
  let input = std::fs::read_to_string("input.txt").unwrap();
  let (_, mut hand_scores) = parse_input_lines(&input, parse_line).unwrap();
  hand_scores.sort_unstable_by(|(hand_a, _), (hand_b, _)| hand_a.cmp(hand_b, false));
  let mut total = hand_scores
    .iter()
    .enumerate()
    .fold(0, |acc, (i, (_, score))| {
      acc + *score as i32 * (i as i32 + 1)
    });
  println!("Total 1: {}", total);
  hand_scores.sort_unstable_by(|(hand_a, _), (hand_b, _)| hand_a.cmp(hand_b, true));
  total = hand_scores
    .iter()
    .enumerate()
    .fold(0, |acc, (i, (_, score))| {
      acc + *score as i32 * (i as i32 + 1)
    });
  println!("Total 2: {}", total);
}
