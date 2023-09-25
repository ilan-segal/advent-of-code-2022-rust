use common;

fn parse_items(items: &str) -> Vec<i32> {
    items
        .lines()
        .map(|item| {
            item.parse::<i32>()
                .expect("Failed to parse item")
        })
        .collect()
}

fn main() {
    let result = common::read_file("input.txt")
        .replace("\r\n", "\n")
        .split("\n\n")
        .map(parse_items)
        .map(|items| items.iter().sum::<i32>())
        .max()
        .expect("Failed to find maximum sum of items");
    println!("{}", result)
}
