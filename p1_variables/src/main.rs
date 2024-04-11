fn main() {
    // mut as mutable
    let mut x: u32 = 1;
    println!("x = {}", x); 
    
    x = 2;
    println!("x = {}", x);

    // can redefine x and even redefine as other types
    let x = x + 1;
    println!("x = {}", x);
    
    {
        // we can use x in diffent scope
        // it won't change the value of x outside this scope 
        // this is called shadowing 
        let x = x - 1;
        println!("x = {}", x);
    }

    // declaring a constant;
    const NUMBER_FIVE: u32 = 5;
    println!("NUMBER_FIVE is {}", NUMBER_FIVE);
    
}
