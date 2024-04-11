use std::io;

fn main() {
    let mut input = String::new();

    // `.expect` deals with the result of read_line
    io::stdin().read_line(&mut input).expect("failed to read line");
    println!("{}", input);
}
