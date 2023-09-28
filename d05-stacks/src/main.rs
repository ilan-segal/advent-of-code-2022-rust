use common;
use regex::Regex;
use std::iter;

fn create_stacks(input: &String) -> Vec<Vec<char>> {
    let line_length = input
        .lines()
        .rev()
        .last()
        .expect("First line of input")
        .len();
    let num_stacks = (1 + line_length) / 4;
    let mut stacks: Vec<Vec<char>> = iter::repeat(vec![])
        .take(num_stacks)
        .collect();
    for line in input
        .lines()
        .take_while(|line| line.contains('['))
    {
        for (i, e) in line
            .chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, e)| e.is_uppercase())
        {
            stacks
                .get_mut(i)
                .expect("Stack in construction")
                .push(e);
        }
    }
    stacks
        .into_iter()
        .map(|stack| {
            stack
                .into_iter()
                .rev()
                .collect::<Vec<char>>()
        })
        .collect()
}

fn read_tops_of_stacks(stacks: &Vec<Vec<char>>) -> String {
    stacks
        .iter()
        .map(|stack| stack.last().expect("Top of stack"))
        .collect()
}

#[derive(Debug)]
struct Move {
    quantity: u8,
    from: usize,
    to: usize,
}

impl Move {
    fn from(line: &str) -> Move {
        let re = Regex::new(r"move (?<quantity>\d+) from (?<from>\d+) to (?<to>\d+)")
            .expect("Regex pattern");
        let Some(captures) = re.captures(line) else {
            panic!("Regex mismatch");
        };
        Move {
            quantity: captures["quantity"]
                .parse::<u8>()
                .expect("quantity"),
            from: captures["from"]
                .parse::<usize>()
                .expect("from"),
            to: captures["to"]
                .parse::<usize>()
                .expect("to"),
        }
    }
}

fn create_moves(input: &String) -> Vec<Move> {
    input
        .lines()
        .skip_while(|line| !line.trim().is_empty())
        .skip(1)
        .map(|line| Move::from(line))
        .collect()
}

fn apply_moves(stacks: &mut Vec<Vec<char>>, moves: &Vec<Move>) {
    for cur_move in moves {
        for _ in 0..cur_move.quantity {
            let item = stacks
                .get_mut(cur_move.from - 1)
                .expect("Access 'from' stack")
                .pop()
                .expect("Pop from stack");
            stacks
                .get_mut(cur_move.to - 1)
                .expect("Access 'to' stack")
                .push(item);
        }
    }
}

fn apply_moves_9001(stacks: &mut Vec<Vec<char>>, moves: &Vec<Move>) {
    for cur_move in moves {
        let from_stack = stacks
            .get_mut(cur_move.from - 1)
            .expect("Access 'from' stack");
        let num_items_in_from = from_stack.len();
        let mut items = from_stack.split_off(num_items_in_from - (cur_move.quantity as usize));
        stacks
            .get_mut(cur_move.to - 1)
            .expect("Access 'to' stack")
            .append(&mut items);
    }
}

fn main() {
    let contents = common::read_file("input.txt");
    let moves = create_moves(&contents);
    let mut part_1_stacks = create_stacks(&contents);
    apply_moves(&mut part_1_stacks, &moves);
    println!("{}", read_tops_of_stacks(&part_1_stacks));
    let mut part_2_stacks = create_stacks(&contents);
    apply_moves_9001(&mut part_2_stacks, &moves);
    println!("{}", read_tops_of_stacks(&part_2_stacks));
}
