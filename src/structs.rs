pub fn mains(){
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let user1 = User {
        active: true,
        username: String::from("h1s3nberg"),
        email: String::from("zhbirlik.2006@gmail.com"),
        sign_in_count: 1,
    };
    println!("Email of user1: {}", user1.email);
}
