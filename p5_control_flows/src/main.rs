fn main() {
    // if else
    let a = 5;
    if a == 5 {
        println!("a is 5");
    } else {
        println!("a is not 5");
    }

    // loop
    let mut counter = 0;
    loop {
        println!("{}", counter);
        counter += 1;
        if counter == 10 {
            break;
        }
    }

    // while loop
    // while counter != 10

    //for loop
    let a : [i32; 5] = [1, 2, 3, 4, 5];
    for element in a.iter() {
        println!("{}!", element)
    }
    for number in 0..5{
        println!("yes");
    }
    
}
