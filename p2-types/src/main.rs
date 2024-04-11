fn main() {
    //intergers
    // i8, i16, i32, i64, i128
    let a: i32 = -1;
    let b: u32 = 2;
    let c: f32 = 1.2;

    //bolean
    let true_false: bool = true;

    //char
    let letter: char = 'a';

    //tuple
    let mut tup: (i32, bool, char) = (1, true, 'a');
    tup.0 = 2;
    println!("{}, {}", tup.0, tup.2);

    //array 
    let arr: [i32; 5] = [1,2,3,4,5];
    println!("{}", arr[1]);


}
