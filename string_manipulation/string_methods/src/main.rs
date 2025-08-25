//String Methods and Ownership

/*
Today, we'll delve deeper into string manipulation by learning various string methods while understanding Rust's unique ownership model. 
By the end of this lesson, you will have a strong grasp of how to manipulate strings and understand how ownership affects strings in Rust.
*/


//String Concatenation: `push_str`

/*
Rust provides multiple ways to concatenate strings. 
The push_str method adds a string slice to the end of another String. 
The variable passed into push_str must be a string literal/slice or a reference to a String. 
Don't forget to declare the String as mutable with mut.

fn main() {
    let mut greeting = String::from("Hello");
    let rust = " Rust";
    greeting.push_str(rust);
    println!("{}", greeting); // Prints: Hello Rust

    let world = String::from(" World!");
    greeting.push_str(&world);
    println!("{}", greeting); // Prints: Hello Rust World!
}
*/

//String Concatenation using `+`
/*
Another way to concatenate strings is by using the + operator. 
This method is slightly different as it moves ownership of the original string. 
In addition, the second variable must be a reference.


fn main() {
    let hello = String::from("Hello, ");
    let rust = "Rust ";
    let explorer = String::from("Explorer!");
    let greeting = hello + &rust + &explorer;
    println!("{}", greeting); // Prints: Hello, Rust Explorer!
    // println!("{}", hello); // Error: `hello` no longer owns "Hello, "
}
*/

//Determining String Length

/*
Rust offers the len method to determine the length of a string. 
This method counts the number of bytes in the string, not the number of characters. 
All ASCII characters are a single byte, so the number of bytes in a string of ASCII characters is equal to the number of characters in the string.

fn main() {
    let s1 = String::from("Hello!");
    let s2 = "Hello, Explorer!";
    println!("The length of s1 is: {}", s1.len()); // Prints: The length of s1 is: 6
    println!("The length of s2 is: {}", s2.len()); // Prints: The length of s2 is: 16
}
*/

//String Equality and Comparison

/*
Rust supports comparing strings for equality using the == operator and inequality using the != operator. 
These operators in Rust are designed to compare different types of strings, making it convenient to check if a string slice and a String contain the same value. 
Keep in mind that comparisons are case sensitive.

fn main() {
    let word1 = "Hello";
    let word2 = "Hello";
    let word3 = "HELLO";
    let word4 = String::from("Hello");

    println!("{}", word1 == word2); // Prints: true
    println!("{}", word1 != word3); // Prints: true
    println!("{}", word1 == word4); // Prints: true
}

*/

//String Comparisons

/*
Besides equality, Rust also allows you to compare strings lexicographically. 
The string that comes first alphabetically is considered "less than" the one that follows. 
So, the string that would come earlier in the dictionary is "less than". 
However, it's important to note that capital letters are considered "less than" lowercase letters because of their order in the Unicode system.


fn main() {
    let apple = String::from("apple");
    let banana = String::from("banana");

    if apple < banana {
        println!("'{}' comes before '{}'", apple, banana); // Prints: 'apple' comes before 'banana'
    } else if apple > banana {
        println!("'{}' comes after '{}'", apple, banana);
    } else {
        println!("'{}' and '{}' are equal", apple, banana);
    }
}

*/

//Practice


/*
fn main() {
    // String Concatenation: push_str
    let mut greeting = String::from("Hello");
    greeting.push_str(" Rust");
    println!("{}", greeting); // Prints: Hello Rust

    let world = String::from(" world!");
    greeting.push_str(&world); // Added reference to make it a valid push_str
    println!("{}", greeting); // Prints: Hello Rust world!

    // String Concatenation: +
    let hello = String::from("Hello, ");
    let rust = "Rust ";
    let explorer = String::from("Explorer!");
    let greeting = hello + &rust + &explorer;
    println!("{}", greeting); // Prints: Hello, Rust Explorer!
    // println!("{}", hello); // Error: `hello` no longer owns "Hello, "

    // Length
    let s1 = String::from("Hello, world!");
    let s2 = "Hello, Explorer!";
    println!("The length of s1 is: {}", s1.len()); // Prints: The length of s1 is: 13
    println!("The length of s2 is: {}", s2.len()); // Prints: The length of s2 is: 16
}

*/

/*
fn main() {
    let mut greeting = String::from("Hello ");
    greeting.push_str("Rust ");
    println!("{}", greeting); // Expected: Hello Rust

    let world = String::from("world!");
    greeting.push_str(&world); 
    println!("{}", greeting); // Expected: Hello Rust world!

    // String Concatenation: +
    let hello = String::from("Hello, ");
    let rust  = "Rust ";
    let explorer = String::from("Explorer!");
    let greeting = hello + rust + &explorer; 
    println!("{}", greeting); // Expected: Hello, Rust Explorer!
}
*/
/*
fn main() {
    let mut intro = String::from("Hi, ");
    // TODO: Using push_str, append the string literal "there"
    intro.push_str("there");
    println!("{}", intro); // Expected: Hi, there
    
    let rustacean = String::from(" Rustacean!");
    // TODO: Using push_str, append rustacean to intro
    intro.push_str(&rustacean);
    println!("{}", intro); // Expected: Hi, there Rustacean!
    // TODO: Print the length of intro
    println!("length of the intro is {}",intro.len());
    
    let beginning = String::from("Hello ");
    let middle = "dear ";
    let ending = String::from("friend.");
    // TODO: Combine strings using +
    
    let welcome = beginning + &middle +&ending;
    println!("{}", welcome); // Expected: Hello dear friend.
    // TODO: Print the length of welcome
        println!("length of the welcome is {}",welcome.len());

}
*/

/*
fn main() {
    // String equality
    let hero1 = "Batman";
    let hero2 = "Batman";
    let hero3 = "Superman";
    let hero4 = String::from("Batman");

    // TODO: Fill in the code to get the correct outputs
    println!("{}", hero1 == hero2); // Expected output: true
    println!("{}", hero1 != hero3); // Expected output: true
    println!("{}", hero1 == hero4); // Expected output: true
    
    // String Comparisons
    let hero5 = String::from("Spiderman");
    let hero6 = String::from("Ironman");

    if hero6 < hero5 {
        println!("{} comes before {}", hero5, hero6); // TODO: Complete the comparison
    } else if hero5 < hero6 {
        println!("{} comes after {}", hero5, hero6); // TODO: Complete the comparison
    } else {
        println!("{} and {} are equal", hero5, hero6);
    }
}
*/

/*
fn main() {
    // String equality
    let hero1 = "Aquaman"; // string literal
    let hero2 = "Wonder Women"; // string literal
    let hero3 = String::from("Aquaman"); // String (heap-allocated)

    println!("{}", hero1 == hero2); // Expected: false
    println!("{}", hero2 != hero3); // Expected: true
    println!("{}", hero1 == hero3); // Expected: true

    // String Comparisons
    let hero4 = String::from("Iron Man"); 
    let hero5 = String::from("Batman"); 

    println!("Does {} comes before {} ? {}", hero4, hero5, hero4 < hero5); 
}

*/