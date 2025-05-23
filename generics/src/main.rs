
//T is a generic type to be specified later
// First argoument is a reference to a array of T
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item
        }
    }

    largest

}

//Example of multiple generics
struct Point<T,U> {
    x: T,
    y: U,
}

fn main() {

    //vec! is a macro that creates a Vec<T> object, using the type of the specified items
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);

    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);

    println!("The largest number is {result}");
}