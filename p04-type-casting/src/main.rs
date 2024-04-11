use std::io;

fn main() {
    // different assignment
    // let x = 12800i32 (so ugly)
    // let x = 12800 as i32;
    let x: i32 = 128000;
    let y: i16 = 10;
    let z = x / (y as i32);
    println!("{}", z);

    //casting a string to int
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read");
    let int_input: i32 = input.trim().parse().unwrap();
    println!("{}", int_input);

}
