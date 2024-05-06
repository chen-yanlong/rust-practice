use std::fs::File;
use std::io::ErrorKind;
use std::io;

fn main() {
    {
        // panic!("crash")
    }
    {
        // enum Result<T, E> {
        //     Ok(T),
        //     Err(E),
        // }

        let f = File::open("hello.txt");
        let f = match f {
            Ok(file) => file,
            Err(error) => match error.kind(){
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("error: {:?}", e),
                },
                other_error => {
                    panic!("Problem opening the file: {:?}", other_error)
                }
            }
        };    
    }
    {
        fn use_question_mark() -> Result<String, io::Error> {
            let f = File::open("hello.txt")?;
            let mut s = String::new();
            Ok(s)
        }
    }


}
