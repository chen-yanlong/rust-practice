fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of {} is {}.", s1, len);

    let mut s2 = String::from("hello2");
    change(&mut s2);
    println!("{}", s2);

    let mut s3 = String::from("hello3");
    let r1 = &mut s3;
    // cannot have another `let r2 = &mut s3`
    // rust prevent one from modifying while another is querying
    // if needed, just take out the `mut`
    println!("{}", r1);

    // let ref_to_nothing = dangle();


}

// s: &String is the reference of s1,
// this is called borrowing 
// refernece are immutable
fn calculate_length(s: &String) -> usize {
    let length = s.len();
    length
}

// to modify a reference, should add a `mut` 
fn change(s2: &mut String) {
    s2.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("no_ref");
//     &s
// }