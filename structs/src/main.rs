struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// FIELDS: each struct OWNS all its data. To make a struct instance reference some data that doesn't own we must set a LIFETIME

//Tuple struct. Useful to create different types with equal data inside, like here
struct ColorRGB(i32,i32,i32);
struct Point3D(i32,i32,i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Implementations block for Rectangle
impl Rectangle {
    //Method of Rectangle
    //Self must be explicited as a reference to the current Rectangle instance calling the method
    //Self reference borrowed as first argoument in a method IS REQUIRED
    //In this case, this is immutable (no mut keyword).
    fn area(&self) -> u32 {
        self.width * self.height //is returned, note no semicolon
    }

    // Multiple argouments example
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height //is returned, note no semicolon
    }

    // This is an Associated Function
    // it returns a Rectangle instance and can be called from the struct name
    // and do not have "&self" as the first parameter
    // (think of a static method in Java)
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {

    //Must me mutable to change his fields' value
    //We cannot tag single fields as mutable
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count:1,
    };

    println!("User 1 {0}",user1.username);

    user1.email = String::from("another@example.com");

    println!("User 1 email:  {0}",user1.email);


    let user2 = build_user(String::from("BARAUS@BARAUS.BARAUS"), String::from("BARAUS"));


    println!("User 12 email:  {0}",user2.email);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 20,
        height: 10,
    };
    let rect3 = Rectangle {
        width: 200,
        height: 100,
    };


    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area() //NOTE: no need to pass rect1 as an argoument
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let rect4: Rectangle = Rectangle::square(4);

    println!(
        "This is a square {:#?}",
        rect4 //NOTE: no need to pass rect1 as an argoument
    );


}

// with this function we can build a User instance.
// note that "email" and "username" parameters have the same name as the struct fields
// this convention is called "field init shorthand" and will make the field passing easer 
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}