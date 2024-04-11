
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

fn main() {
    let user1 = build_user(
        String::from("dinoxxur@gmail.com"), 
        String::from("alan"),
    );

    let user2 = User {
        email: String::from("alex@gmail.com"),
        username: String::from("alex"),
        ..user1 // give the rest of the param whatever user1 has
    };   

}

fn build_user(email: String, username:String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
