fn main() {
    // Scalar types

    // Unsigned integer (u8)
    let num: u8 = 5;  // An unsigned 8-bit integer
    println!("Unsigned 8-bit integer (u8): {}", num);

    // Signed integer (i8)
    let temp: i8 = -5;  // A signed 8-bit integer
    println!("Signed 8-bit integer (i8): {}", temp);

    // Floating-point number (f32)
    let pi: f32 = 3.14159;  // A 32-bit floating point number
    println!("Floating-point number (f32): {}", pi);

    // Boolean (bool)
    let is_active: bool = true;  // A boolean value
    println!("Boolean (bool): {}", is_active);

    // Character (char)
    let letter: char = 'A';  // A single character
    println!("Character (char): {}", letter);

    // String (String)
    let greeting: String = String::from("Hello, Rust!");  // A dynamic string
    println!("String (String): {}", greeting);

    // Compound types

    // Tuple
    let person: (String, i32, f32) = ("Alice".to_string(), 30, 5.5);  // A tuple with a string, integer, and float
    println!("Tuple (person): Name: {}, Age: {}, Height: {}", person.0, person.1, person.2);

    // Array
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];  // An array of integers
    println!("Array (numbers): First number: {}", numbers[0]);

    // Vector (Vec)
    let mut vec = Vec::new();  // A mutable vector
    vec.push(10);  // Adding an element to the vector
    vec.push(20);
    println!("Vector (vec): First element: {}", vec[0]);

    // Option type
    let some_number: Option<i32> = Some(5);  // A valid number inside Option
    let no_number: Option<i32> = None;  // No value inside Option

    match some_number {
        Some(n) => println!("Option (some_number): The number is: {}", n),
        None => println!("Option (some_number): No number available"),
    }

    match no_number {
        Some(n) => println!("Option (no_number): The number is: {}", n),
        None => println!("Option (no_number): No number available"),
    }
}
