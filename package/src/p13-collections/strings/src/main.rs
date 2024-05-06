// to use graphemes
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    //methods to append string
    {
        let mut s1 = String::from("hello");
        s1.push_str(" world");
        s1.push('!');
        let s2 = String::from("alan");
        // takes ownership of s1
        let s3 = s1 + &s2;
        println!("{}", s3);    
        // does not takes ownership
        let s4 = format!("{}{}", s3, s2);
        println!("{}", s4)
    } 
    // indexing strings
    {
        let hello: String = String::from("Hello");
        for i in hello.graphemes(true) {
            println!("{}", i);
        }
    }
}
