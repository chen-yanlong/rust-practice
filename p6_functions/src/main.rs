fn main() {
    let result = add_numbers(3, 5);
    println!("{}", result);
}

fn add_numbers(a: i32, b: i32) -> i32 {
    // no semicolon!!
    a + b
    // or return a + b;
}
