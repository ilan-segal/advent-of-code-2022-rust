use common;
use std::collections::HashSet;

#[derive(Clone)]
struct Rucksack {
    left: Vec<char>,
    right: Vec<char>,
}

impl Rucksack {
    pub fn get_odd_item(self: Self) -> char {
        let left_set: HashSet<char> = HashSet::from_iter(self.left);
        *self
            .right
            .iter()
            .filter(|item| left_set.contains(item))
            .last()
            .expect("Could not find common item")
    }

    pub fn new(items: String) -> Rucksack {
        let count: usize = items.chars().count();
        assert!(count % 2 == 0);
        let halfway = count / 2;
        Rucksack {
            left: items
                .chars()
                .into_iter()
                .take(halfway)
                .collect(),
            right: items
                .chars()
                .into_iter()
                .skip(halfway)
                .collect(),
        }
    }

    pub fn get_all_items(self: &Self) -> Vec<char> {
        self.left
            .clone()
            .into_iter()
            .chain(self.right.clone().into_iter())
            .collect()
    }
}

fn get_item_priority(item: char) -> u32 {
    if item.is_lowercase() {
        item as u32 - 'a' as u32 + 1
    } else {
        item as u32 - 'A' as u32 + 27
    }
}

fn get_groups(rucksacks: &Vec<Rucksack>) -> Vec<[&Rucksack; 3]> {
    let mut groups = vec![];
    for n in (0..rucksacks.len()).step_by(3) {
        groups.push([
            rucksacks.get(n).unwrap(),
            rucksacks.get(n + 1).unwrap(),
            rucksacks.get(n + 2).unwrap(),
        ]);
    }
    return groups;
}

fn get_group_badge(group: [&Rucksack; 3]) -> char {
    group
        .into_iter()
        .map(|r| r.get_all_items())
        .map(|items| HashSet::from_iter(items))
        .reduce(|a: HashSet<char>, b: HashSet<char>| a.intersection(&b).cloned().collect())
        .unwrap()
        .into_iter()
        .last()
        .unwrap()
}

fn main() {
    let rucksacks: Vec<Rucksack> = common::read_file("input.txt")
        .lines()
        .map(String::from)
        .map(Rucksack::new)
        .collect();
    let badges = get_groups(&rucksacks)
        .into_iter()
        .map(|group| get_group_badge(group));
    println!(
        "{}",
        rucksacks
            .clone()
            .into_iter()
            .map(|r| r.get_odd_item())
            .map(|item| get_item_priority(item))
            .sum::<u32>()
    );
    println!(
        "{}",
        badges
            .into_iter()
            .map(|badge| get_item_priority(badge))
            .sum::<u32>()
    );
}
