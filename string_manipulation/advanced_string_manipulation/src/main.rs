/*
Building on the previous knowledge of basic string operations, 
you will learn how to find substrings, check for the presence of a substring within another string, 
replace parts of a string, and transform strings using splitting and joining techniques.

*/

//Finding a Substring

/*
In Rust, you can use the string method .find() to locate the position of a substring within another string. 
This can be particularly useful when you need to determine whether a certain pattern exists in your text. 
The .find() method in Rust returns an Option<usize>, and it can yield two types of values:


Some(index)
This variant signifies that the substring was found within the string.
index is the starting position (0-based index) of the first occurrence of the substring within the string.
None
This variant indicates that the substring was not found within the string.


fn main() {
    let s = String::from("Hello, world!");
    match s.find("world") {
        Some(index) => println!("Found 'world' at index: {}", index),  
        None => println!("'world' not found")
    }
    // Prints: "Found 'world' at index: 7"
    
    match s.find("Rust") {
        Some(index) => println!("Found 'Rust' at index: {}", index),
        None => println!("'Rust' not found")
    }
    // Prints: 'Rust' not found
}
*/

//Checking for Substring Presence

/*
Another common task is to check if a substring exists within a string using the .contains() method. 
It returns a boolean value, which you can use to conditionally execute parts of your code.


fn main() {
    let s = String::from("Hello, world!");
    if s.contains("world") {
        println!("The string contains 'world'");  // Prints: "The string contains 'world'"
    } else {
        println!("The string does not contain 'world'");
    }
}

*/

//Replacing Substrings

/*
Replacing parts of a string is another essential string operation. 
Rust provides the .replace() method to substitute occurrences of a substring with another value.


fn main() {
    let s = String::from("Hello, world!");
    let new_s = s.replace("world", "Rust");
    println!("{}", new_s);  // Prints: "Hello, Rust!"
}

*/

//Splitting and Joining Strings
/*
Splitting and joining strings are powerful techniques for transforming textual data. 
The .split() method divides a string into a vector of substrings based on a delimiter, 
and join can be used to combine a vector of strings into a single string.


fn main() {
    let s = String::from("apple, banana, pear");
    let fruits: Vec<&str> = s.split(',').collect();
    println!("{:?}", fruits);  // Prints: ["apple", " banana", " pear"]

    let joined = fruits.join(" & ");
    println!("{}", joined);  // Prints: "apple &  banana &  pear"
}

*/

//Practice

/*
fn main() {
    // Find
    let s = String::from("Wonder Woman, Batman, Iron Man");
    match s.find("Batman") {
        Some(index) => println!("Found 'Batman' at index: {}", index),
        None => println!("'Batman' not found")
    }

    // Contains
    if s.contains("Wonder Woman") {
        println!("The string contains 'Wonder Woman'");
    } else {
        println!("The string does not contain 'Wonder Woman'");
    }

    // Replace
    let new_s = s.replace("Iron Man", "Superman");
    println!("{}", new_s);
}
*/

/*
fn main() {
    // Check for substring presence
    let heroes = String::from("Superman, Batman, Wonder Woman");
    if heroes.contains("Superman") {
        println!("The string contains 'Superman'");
    } else {
        println!("The string does not contain 'Superman'");
    }
}

*/


/*
fn main() {
    // Find
    let s = String::from("Spider-Man, Hulk, Thor");
    
    // TODO: Use the find method to locate the position of "Hulk"
    match s.find("Hulk") {
        Some(index) => println!("Found 'Hulk' at index: {}", index),
        None => println!("'Hulk' not found")
    }
}
*/

/*
fn main() {
    let hero = String::from("Batman is protecting Gotham with Batman's gadgets.");
    
    // TODO: Replace occurrences of "Batman" with "Nightwing"
    let new_hero = hero.replace("Batman","Nightwing");
    
    println!("{}", new_hero); // Expected: "Nightwing is protecting Gotham with Nightwing's gadgets."
}
*/

/*
fn main() {
    // Splitting the string
    let s = String::from("Spider-Man, Iron Man, Captain Marvel");
    // TODO: Split the string s by commas and collect into a vector
    let heroes: Vec<&str> = s.split(",").collect();
    println!("{:?}", heroes);  // Should print: ["Spider-Man", "Iron Man", "Captain Marvel"]
    
    // Joining the string
    // TODO: Join the vector of superheroes with " & " and store in joined_heroes
    let joined_heroes =heroes.join(" & ");
    println!("{}", joined_heroes);  // Should print: "Spider-Man & Iron Man & Captain Marvel"
}
*/