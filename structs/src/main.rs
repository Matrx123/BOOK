struct User {
    name: String,
    email: String,
    active: bool,
}

fn main() {
    let user1 =  User {
        name: String::from("Vipin Joshi"),
        email: String::from("vpnjsh55@gmail.com"),
        active: true,
    };
    println!("User 1 :: {:?}, {:?}, {:?}", user1.name, user1.email, user1.active);

    let user2 = User {
        email: String::from("vipinj.ingenious@gmail.com"),
        ..user1
    };
    println!("User 2 :: {:?}, {:?}, {:?}", user2.name, user2.email, user2.active);

    let new_user = build_user(String::from("vipinj@yopmail.com"), String::from("tipin yoshi"));
    println!("Build user :: {:?}, {:?}, {:?}", new_user.name, new_user.email, new_user.active);

}

fn build_user(email : String, username: String) -> User {
    User {
        name : username,
        email: email,
        active: true,
    }
}