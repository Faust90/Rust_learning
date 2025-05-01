enum IpAddrKind {
    V4,
    V6,
}

// Here we have an enum with data attached for each value
// This means that each variant is a function that creates
// a new istance
enum IpAddr {
    V6(String),
    V4(String),
}

#[derive(Debug)] // so we can inspect the state 
enum State {
    Alaska,
    Alabama,
    NewYork,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

fn main() {
    {
        let _six = IpAddrKind::V6;
        let _four = IpAddrKind::V4;

        let _home = IpAddr::V4(String::from("127.0.0.1"));
        let _loopback = IpAddr::V6(String::from("::1"));
    }

    {
        //These are nullable values. Rust does not work with nullable per se, so we must use "Option<T>" enum
        let _some_number = Some(5);
        let _null_number: Option<i32> = None;

        // Non working code: i8 and  Option<i8> are different types, cannot interct easily
        // let x: i8 = 5;
        // let y: Option<i8> = Some(5);

        // let sum = x + y;
    }

    {
        println!(
            "value_in_cents {0}",
            value_in_cents(Coin::Quarter(State::Alaska))
        );

        //using Option for summing nullable numbers
        plus_one(Some(4));
        plus_one(None);
    }

    {
        //3 as unsiged 8 bit number
        let configured_max = Some(3u8);
        let configured_max_none: Option<u8> = None;

        // _ is a catchall (think of "default" in Java switch). We return an empty tuple whenever max is NONE or not a Some(u8)
        match configured_max {
            Some(max) => println!("Maximum is configured to {max}"),
            _ => (),
        }

        //This sintax is equivalent to the previous one
        // note THE "=" operator
        // I find this syntax uglier and unclearer
        if let Some(max) = configured_max {
            println!("Maximum is configured to {max}");
        }

        match configured_max_none {
            Some(max) => println!("Maximum is configured to {max}"),
            _ => (),
        }
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    //Think of a Java switch
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter for {state:?}");
            25 // value is returned, NOTE NO SEMICOLON
        }
    }
}

// if x is None, will return None. In Some(number), return Some(number +1)
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(x) => Some(x + 1),
        None => None,
    }
}
