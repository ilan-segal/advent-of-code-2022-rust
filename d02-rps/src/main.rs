use common;
use std::{cmp::Ordering, str::Lines};

#[derive(PartialEq, Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let result = match self {
            Hand::Rock => match other {
                Hand::Rock => Ordering::Equal,
                Hand::Paper => Ordering::Less,
                Hand::Scissors => Ordering::Greater,
            },
            Hand::Paper => match other {
                Hand::Rock => Ordering::Greater,
                Hand::Paper => Ordering::Equal,
                Hand::Scissors => Ordering::Less,
            },
            Hand::Scissors => match other {
                Hand::Rock => Ordering::Less,
                Hand::Paper => Ordering::Greater,
                Hand::Scissors => Ordering::Equal,
            },
        };
        Option::Some(result)
    }
}

#[derive(Clone, Copy)]
struct Match {
    opponent: Hand,
    you: Hand,
}

impl Match {
    pub fn get_score(self) -> u32 {
        self.get_hand_score() + &self.get_outcome_score()
    }

    fn get_hand_score(self: &Self) -> u32 {
        match self.you {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    fn get_outcome_score(self: &Self) -> u32 {
        let ordering = self
            .you
            .partial_cmp(&self.opponent)
            .expect("Cannot compare hands.");
        match ordering {
            Ordering::Less => 0,
            Ordering::Equal => 3,
            Ordering::Greater => 6,
        }
    }

    fn as_outcome(self: &Self) -> Match {
        let desired_outcome = match self.you {
            Hand::Rock => Ordering::Less,
            Hand::Paper => Ordering::Equal,
            Hand::Scissors => Ordering::Greater,
        };
        Match {
            opponent: self.opponent,
            you: *[Hand::Rock, Hand::Paper, Hand::Scissors]
                .iter()
                .filter(|hand| {
                    hand.partial_cmp(&&self.opponent)
                        .unwrap()
                        == desired_outcome
                })
                .last()
                .unwrap(),
        }
    }
}

fn construct_matches(lines: Lines) -> Vec<Match> {
    lines
        .map(|line| (line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap()))
        .map(|(first, second)| Match {
            opponent: match first {
                'A' => Hand::Rock,
                'B' => Hand::Paper,
                'C' => Hand::Scissors,
                _ => panic!("Unrecognized symbol {}", first),
            },
            you: match second {
                'X' => Hand::Rock,
                'Y' => Hand::Paper,
                'Z' => Hand::Scissors,
                _ => panic!("Unrecognized symbol {}", second),
            },
        })
        .collect()
}

fn main() {
    let input = common::read_file("input.txt");
    let matches = construct_matches(input.lines());
    println!(
        "{}",
        matches
            .iter()
            .map(|m| m.get_score())
            .sum::<u32>()
    );
    println!(
        "{}",
        matches
            .iter()
            .map(|m| m.as_outcome())
            .map(|m| m.get_score())
            .sum::<u32>()
    );
}
