use common;

struct Range {
    start: u16,
    end: u16,
}

impl Range {
    pub fn new(string: &str) -> Range {
        let (start, end) = string
            .split_once('-')
            .expect("Failed to construct range");
        Range {
            start: start
                .parse::<u16>()
                .expect("Failed to parse start"),
            end: end
                .parse::<u16>()
                .expect("Failed to parse start"),
        }
    }
}

fn fully_contains(a: &Range, b: &Range) -> bool {
    (a.start <= b.start && b.end <= a.end) || (b.start <= a.start && a.end <= b.end)
}

fn overlaps(a: &Range, b: &Range) -> bool {
    (a.start <= b.start && b.start <= a.end) || (a.start <= b.end && b.end <= a.end)
}

fn main() {
    let range_pairs: Vec<(Range, Range)> = common::read_file("input.txt")
        .lines()
        .map(|line| {
            line.split_once(',')
                .expect("Failed to split line")
        })
        .map(|(s1, s2)| (Range::new(s1), Range::new(s2)))
        .collect();
    println!(
        "{}",
        range_pairs
            .iter()
            .filter(|(a, b)| fully_contains(a, b))
            .count()
    );
    println!(
        "{}",
        range_pairs
            .iter()
            .filter(|(a, b)| overlaps(a, b))
            .count()
    );
}
