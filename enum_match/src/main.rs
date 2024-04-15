#[derive(Debug)]
enum State {
    Alabama,
    Alaska,
    Georgia,
    Florida,
}

enum Coin {
    Nickle,
    Dime,
    Quarter(State),
    Penny,
}

fn main() {
    let result = value_in_centes(Coin::Quarter(State::Alabama));
    println!(":: result : {:?}", result);
}

fn value_in_centes(coin:Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny");
            1
        },
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
