use common;

fn parse_items(items: &str) -> u32 {
    items
        .lines()
        .map(|item| {
            item.parse::<u32>()
                .expect("Failed to parse item")
        })
        .sum()
}

fn main() {
    let mut totals: Vec<u32> = common::read_file("input.txt")
        .replace("\r\n", "\n")
        .split("\n\n")
        .map(parse_items)
        .collect();
    totals.sort();
    println!(
        "{}",
        totals
            .last()
            .expect("Failed to get maximum element")
    );
    println!("{}", totals.iter().rev().take(3).sum::<u32>());
}
