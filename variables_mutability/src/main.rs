fn main() {
    // x is an immutable i32 number of 5
    let x = 5;

    // Previous x is shadowed by a new x with value 5+1 = 6
    // x is still immutable!
    let x = x + 1;
    {
        // Previous x is shadowed (only inside this scope) by a new x with value 6 * 2 = 12
        // x is still immutable!
        let x = x * 2;
        println!("The value of x in the inner scope is {x}");
    }

    println!("The value of x is {x}");

    // Error on compile: spaces is a mutable string, cannot mutate in a numeric value
    // let mut spaces = "   ";
    // spaces = spaces.len();

    // Immutable "spaces" is shadowed from string to number
    let _spaces = "   ";
    let _spaces = _spaces.len();

    // Tuples
    let tuple = (1, 1, 0);
    
    println!("The value of first value in tuple is {0}",tuple.0);
}
