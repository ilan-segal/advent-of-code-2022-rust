use common;
use itertools::Itertools;

fn is_unique(message: &str) -> bool {
    message.chars().unique().count() == message.len()
}

fn find_first_unique_slice(message: &String, slice_length: usize) -> usize {
    (0..message.len())
        .filter_map(|i| {
            if i + slice_length <= message.len() {
                Some((i + slice_length, &message[i..i + slice_length]))
            } else {
                None
            }
        })
        .filter_map(|(i, slice)| if is_unique(slice) { Some(i) } else { None })
        .next()
        .expect("Cannot find unique substring")
}

fn main() {
    let message = common::read_file("input.txt");
    println!("{}", find_first_unique_slice(&message, 4));
    println!("{}", find_first_unique_slice(&message, 14));
}
