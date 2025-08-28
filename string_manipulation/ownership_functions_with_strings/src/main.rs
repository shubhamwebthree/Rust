// Ownership and Functions with Strings

/*
we'll dive deeper into the heart of Rust’s memory safety model. 
Additionally, we'll explore how ownership plays a role when passing data to functions.
Understanding these concepts is crucial as they form the foundation of Rust programming.

*/

//Ownership Review

/*
Rust's ownership model ensures memory safety without needing a garbage collector. 
When a variable in Rust goes out of scope, it is automatically cleaned up. 
This model has three main rules

Each value in Rust has a single owner.
The value is dropped when the owner goes out of scope.
Ownership can be transferred to another variable.

fn main() {
    let s1 = String::from("Hello");
    let s2 = s1;
    println!("{}", s2); // Prints: Hello
    // println!("{}", s1); // Error: value borrowed here after move
}
*/

//Cloning Data

/*
Sometimes, instead of transferring ownership, we want to create a deep copy of the data. 
This is done using the clone method

fn main() {
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("{}", s1); // Prints: Hello
    println!("{}", s2); // Prints: Hello
}
*/


//Functions: Transferring Ownership

/*
When we pass a variable to a function, we can transfer ownership to the function

fn main() {
    let s = String::from("Hello");
    takes_ownership(s); // s is moved here and can no longer be used
    // println!("{}", s); // Error: value borrowed here after move
}

fn takes_ownership(some_string: String) {
    println!("Taking ownership of: {}", some_string); // Prints: Taking ownership of: Hello
}

*/

//Functions: Passing References

/*
To avoid moving ownership, we can pass a reference to the function:

fn main() {
    let s = String::from("Hello");
    let length = calc_length(&s);
    println!("The length of {} is {}", s, length); // Prints: The length of Hello is 5
}

fn calc_length(s: &String) -> usize {
    s.len()
}

*/

//Functions: Passing Mutable References

/*
Mutable references allow us to modify data without transferring ownership:

fn main() {
    let mut s = String::from("Hello");
    change_string(&mut s);
    println!("{}", s); // Prints: Hello Rust Explorer
}

fn change_string(some_string: &mut String) {
    let rust = " Rust";
    let explorer = String::from(" Explorer");
    some_string.push_str(rust);
    some_string.push_str(&explorer);
}
*/

//Practice

/*
fn main() {
    // Functions: Transferring Ownership
    let villain = String::from("Joker");
    takes_control(villain); // villain is moved here and can no longer be used
    // println!("{}", villain); // Error

    // Functions: Passing References
    let hero_quote = String::from("With great power...");
    let quote_len = get_length(&hero_quote);
    println!("The length of the quote {} is {}", hero_quote, quote_len);

    // Functions: Passing Mutable References
    let mut hero_title = String::from("Super");
    add_word(&mut hero_title);
    println!("{}", hero_title); // Prints: Super Hero
}

fn takes_control(character: String) {
    println!("Taking control of: {}", character);
}

fn get_length(text: &String) -> usize {
    text.len()
}

fn add_word(word: &mut String) {
    let addition = " Hero";
    word.push_str(addition);
}
*/

/*
fn main() {
    // Ownership Basics
    let hero = String::from("Cosmo");
    let hero_copy = hero.clone();
    println!("I am the original {}", hero);
    println!("I am the cloned {}", hero_copy);
}
*/

/*
fn main() {
    let hero_quote = String::from("I am Iron Man.");
    // TODO: Call your function to get the quote length and store it in a variable named quote_length
    let quote_length = get_quote_length(&hero_quote);
    println!("The length of '{}' is {}", hero_quote, quote_length);
    
}

// TODO: Create a function called get_quote_length that returns the length of a String
fn get_quote_length(quote: &String) -> usize {
    quote.len()
}
*/

/*
fn main() {
    let mut superhero = String::from("Spider");
    update_name(& mut superhero); 
    println!("{}", superhero); // Expected: Spider Man
}

fn update_name(name: &mut String) { 
    let title = " Man";
    name.push_str(title);
}
*/

/*
fn main() {
    let mut title = String::from("Wonder");
    // TODO: Call your function
    add(&mut title);
    println!("{}", title); // Expected: Wonder Woman
}

// TODO: Create a function that adds " Woman" to an input String
fn add(hero: &mut String){
    let name = " Woman";
    hero.push_str(name);
}

*/

