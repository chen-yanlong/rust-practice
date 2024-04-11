enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6,
}

enum Message {
    Quit, 
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn some_function() {
        println!("something")
    }
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

//Option enum
// enum Option<T> {
//     Some(T),
//     None,
// }


fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // let localhost = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1")
    // }
    let localhost = IpAddrKind::V4(127, 0, 0, 1);

    // Option enum
    let five = Some(5);
    let null: Option<i32> = None;
    let x = 5;
    // Option method `unwrap_or`
    let sum = x + null.unwrap_or(0);



}
