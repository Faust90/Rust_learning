fn main() {
    let s1: String = String::from("Hello");

    // reference of s1 is passed (Borrowing)
    let len = calculate_length(&s1);

    // Note that we can use s1 without worrying about the ownership being moved inside calculate_length
    println!("The length of {s1} is {len}.");

    let mut s2 = String::from("Hello");

    {
        let reference: &mut String = &mut s2;
        // let second_reference = &mut s2; // This gives error: mutable reference can be borrowed only one time per scope

        // reference of s2 is passed and is mutable
        change(reference);
    }

    {
        let _reference = &s2;
        //let second_reference = &mut s2; // This gives error: a non mutable reference is already been borrowed, unless...
    }

    {
        let reference: &String = &s2;
        println!("{reference}");

        //Here we see that a second mutable reference CAN be used even if a first NON mutable one has been borrowed.
        // Why? reference here is out of scope, it has been "consumed" by println!("{reference}")
        let second_reference = &mut s2; 
        println!("{second_reference}");
    }


    println!("{s2}");
    
    // This code is just for learning purposes
    // let reference_to_nothing = dangle();
}

//usize is how many bytes it takes to reference something (in this case a String) in memory
fn calculate_length(s: &String) -> usize {
    //&String is passed by reference. This "tells" to calculate_length to NOT take ownership

    // By default, we cannot modify a borrowed variable (think of it like Java parameter with "final" keyword )
    // s.push_str(", world"); This gives error

    s.len() //Returned value, note missing semicolon
} // the value of s is not dropped because we borrowed it through reference

fn change(s: &mut String) {
    //&String is passed by mutable reference.

    s.push_str(", world");
}

// Commented because will not compile
// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s //Reference is returned, note the missing semicolon. However, the value is now dropped. This reference points to nothing!
// }