/*
In Rust, we have various data types that represent numbers, truth values, characters, and more. 
we will focus on i32, f32, bool, char, and String.

variables must be surrounded by single quotes (')
*/


// The i32 data type represents 32-bit integers. 
//The maximum value an i32 can store is 2147483647, or 2^31 −1, and the minimum is -2147483648
let days_in_week: i32 = 7;
let maximal_integer: i32 = 2147483647;

//f32
// f32 data type in Rust, used for floating-point numbers It can contain up to 7 decimal digits
let pi: f32 = 3.141592;

//bool
// we have the bool data type, This type is commonly used in logical expressions and decision making
let is_earth_round: bool = true;
println!("{}", is_earth_round);  // This will print: true

let is_earth_flat: bool = false;
println!("{}", is_earth_flat); // This will print: false

//char
// char data type, This type is used to represent single Unicode character.
let first_letter_of_alphabet: char = 'A';
println!("{}", first_letter_of_alphabet); // This will print: A


//String
//There are two types of Strings in Rust.
//String literals are immutable and have a type of &str.

let welcome1 = "Hello World!"; // Creates a string literal
println!("{}", welcome1); // This will print: Hello World!


// On the other hand, a String is by default immutable, but can be made mutable using the mut keyword.

let mut welcome2 = String::from("Welcome to Rust!"); // Creates a new String
println!("{}", welcome2); // This will print: Welcome to Rust!

welcome2 = String::from("Hello Rust World!"); // Changing the value of welcome2
println!("{}", welcome2); // This will print: Hello Rust World!





// code practice 
/*
fn main() {
    
    // TODO: Declare a f32 variable to store the average temperature on Earth in Fahrenheit - 57.2 degrees
    let average_temperature_on_earth:f32 = 57.2;
    
    // TODO: Declare an i32 variable to store the number of days in a year on Earth - 365 days
    let days_in_year :i32 = 365;
    
    // TODO: Print the average temperature on Earth followed by " degrees Fahrenheit."
    println!("average temperature on Earth is {:.1}", average_temperature_on_earth);
    
    // TODO: Print the number of days in a year on Earth, followed by " days in a year on Earth."
    println!("no of days in a year {}",days_in_year);
    
    // TODO: Declare a boolean variable to store the fact whether water freezes at 32 degrees Fahrenheit - it does!
    let water_freeze_at_32_degree :bool = true; 
    
    // TODO: Print whether water freezes at 32 degrees Fahrenheit
    println!("wether wather freezes at 32 degree {}", water_freeze_at_32_degree);
    
    // TODO: Use a char variable to represent the first letter of 'Earth' - E
    let first_letter_of_earth : char = 'E';
    
    // TODO: Print the first letter of the 'Earth'
    println!("First letter of earth is {}", first_letter_of_earth);
}
*/