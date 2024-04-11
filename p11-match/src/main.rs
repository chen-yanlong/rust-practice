enum Coin {
    Ten,
    Fifty,
    Hundred,
    Thousand,
}

fn main() {
   
}

fn value(coin: Coin) -> u16 {
    match coin {
        Coin::Ten => 10,
        Coin:: Fifty => 50,
        // Coin::Hundred => 100,
        // Coin::Thousand => 1000,
        _ => 100, //others, do this
    }
}

// if let expression(so weird)
fn if_let(some_number: Option<i8>) {
    // enums have to list all posibilities,
    // so here comes the if let
    // the below means: if some_number is a Option enum with value Some(3), print
    if let Some(3) = some_number {
        println!("its 3!")
    }

}