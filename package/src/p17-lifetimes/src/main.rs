fn main() {
    let r;
    {
        let x = 5;
        r = &x;        
    }
    //println!("{}", r); => error

    let string1 = String::from("hello");
    let longer;
    {
        let string2 = String::from("hellooo");
        longer = largest(&string1, &string2);
    }
    // println!("{}", longer); => err, string2 does not live 'till here


    let s : &'static str = "static lifetime lives all over the program";

}

//generic lifetime 
// 'a 'b 'everything
// the return lifetime will be the smallest between x and y
fn largest<'a>(x: &'a str, y : &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// struct lifetime
struct ImportantExcerpt<'a>{
    part: &'a str,
}

// life time rules:
// 1. each param gets its own lifetime parameter
// 2. one input, that lifetime is assigned to output lifetime 
// 3. if there are &self as input, the lifetime of self will be assign to all output 
impl<'a> ImportantExcerpt<'a> {
    fn return_part(&self, announcement: &str) -> &str {
        println!("attention: {}", announcement);
        self.part
    }   
}