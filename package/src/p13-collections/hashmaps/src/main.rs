use std::collections::HashMap;

fn main() {
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();

    // this will take ownership of blue and yellow
    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("{:?}", score);

    //insert
    scores.insert(String::from("Blue"), 20);

    // or_insert: insert if there exist this key, do nothing otherwise
    scores.entry(String::from("Yellow")).or_insert(40); 
    //this does nothing because Yellow has already exist

    
}
