// This struct can hold a reference. 
// A instance of this struct cannot outlive the lifecycle ('a) of the reference field (part)
struct ImportantExcerpt<'a> {
    part: &'a str,
}

//'static lifetime can endure for all the program's life
let s: &'static str = "I have a static lifetime.";

fn main() {
    //This code shows a WRONG usage of life times
    // {
    //     let r;                  // ---------+-- 'a
    //                             //          |
    //     {                       //          |
    //         let x = 5;          // -+-- 'b  |
    //         r = &x;             //  |       |
    //     }                       // -+       |
    //                             //          |
    //     //Here x is deleted     //          |
    //     //and deallocated.      //          |
    //     //&x does not point     //          |
    //     //at anything!          //          |
    //     //this gives            //          |
    //     //compile error         //          |
    //                             //          |
    //     println!("r: {r}");     //          |
    // }                           // ---------+


    //This code shows a correct usage of lifetime
    {
        let x = 5;              // ----------+-- 'b
                                //           |
        let r = &x;             // --+-- 'a  |
                                //   |       |
        println!("r: {r}");     //   |       |
                                // --+       |
                                //           |
    }                           // ----------+

    {
        let first_sentence;
        let novel = String::from("Call me Ishmael. Some years ago...");
        first_sentence = novel.split('.').next().unwrap();
        let i = ImportantExcerpt {
            part: first_sentence,
        };
    }
}