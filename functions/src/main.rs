fn main() {
    another_function(4);

    
    another_function({
        let x = 5;
        x + 1 //NOTE: no semicolon here, this makes the statement return the value of x +1 
        // ALTERNATIVE SYNTAX: return x + 1;
    });
    
    another_function(five());

    another_function(plus_one(7));

}

fn another_function(x: i32){

    println!("The value of x is: {x}");
}


// Return five with -> 
fn five() -> i32 { 5 }

fn plus_one(x: i32) -> i32 {
    x + 1 //NOTE no semicolon here!
    // ALTERNATIVE SYNTAX: return x + 1;
}