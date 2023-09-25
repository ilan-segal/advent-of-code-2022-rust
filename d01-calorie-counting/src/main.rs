use common;

fn main() {
    let lines = common::read_file("input.txt");
    for line in lines {
        println!("{}", line);
    }
}
