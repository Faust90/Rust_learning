fn main() {
    // Note that string literals ( let mut s = "Hello" ) cannot be concatenated;
    {
        let mut hello = String::from("Hello");

        hello.push_str(", world");

        println!("{hello}");
    }
    //Here Rust calls "drop" and frees "s", like C++ RAII

    // s1 and s2 points the same value!
    {
        let _s1 = String::from("hello");
        let s2: String = _s1; //from here, s1 is no longer a valid reference. the value has "moved" to s2 

        // Next line is commented, or else will rise a owneship compile error
        // println!("{s1}, world!");
        // This line will not
        println!("{s2}, world!");
    }

    {
        let mut s = String::from("hello"); // Warning from unused value
        s = String::from("ahoy"); // "hello" data is not referenced anymore, now is free

        println!("{s}, world!");
    }

    // s1 and s2 points at to equal values with different pointers in memory
    {
        let s1 = String::from("hello");
        let s2 = s1.clone();

        println!("s1 = {s1}, s2 = {s2}");

        // x and y points at to equal values with different pointers in memory
        // this is because integers have known initial size, so Rust knows it is possible to quickly copy
        // long story short: integers in Rust implement the "Copy" Trait
        let x = 5;
        let y = x;

        println!("x = {x}, y = {y}");
    }

    {
        let s = String::from("Hello");

        takes_ownership(s); // s value is moved into function

        // s is no longer valid here
        // println!("{s}, world!"); // this line gives ERROR!

        let x = 8;

        makes_copy(x); // i32 implements Copy Trait. x is NOT MOVED inside the fuction, it is copied (C++ passed by value)

        println!("x = {x}");
    }

    {
        let s1: String = gives_ownership(); // s1 receive ownership from gives_ownership (move)

        println!("s1 = {s1}");

        let s2 = String::from("Hello");

        let s3 = takes_and_gives_back(s2);

        println!("s3 = {s3}");
    }
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // some_string goes out of scope, drop is called and memory is freed. Whatever was passed in some_string is not available anymore

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string // some_string goes out of scope and moved to the target variable
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    a_string // a_string is returned back (note missing semicolon) and moved out the fuction
}

fn makes_copy(some_int: i32) {
    // some_int comes into scope
    println!("{some_int}");
} // some_int goes out of scope. Nothing happens (no drop call)
