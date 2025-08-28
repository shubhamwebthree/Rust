//String Conversions and Methods

/*
build on that knowledge by learning how to convert data to and from strings, and how to manipulate strings using various methods. 

Rust provides robust tools for string manipulation, making it easier to convert different data types to and from strings and apply various string methods, 
such as changing case, trimming spaces, and handling escape characters.

String Conversions

Converting data to and from strings is a common necessity in programming, and Rust offers several utilities to perform these conversions seamlessly.
Here’s an example to demonstrate how to convert to and from string literals, String`, and numbers.


fn main() {
    let data = "initial contents";
    let s1 = data.to_string(); 
    println!("String: {}", s1);  // Prints: "String: initial contents"

    let s2 = String::from("Hello, world!");
    let s2_literal = s2.as_str();
    println!("{}", s2_literal);  // Prints: "Hello, world!"

    let num = 42;
    let num_str = num.to_string();
    println!("String: {}", num_str);  // Prints: "String: 42"

    let parsed_num: i32 = num_str.parse().unwrap();
    println!("Number: {}", parsed_num);  // Prints: "Number: 42"
}


data.to_string() converts a string literal to a String type.
s2.as_str() converts a String to a string literal
num.to_string() converts an integer to a string.
num_str.parse() converts the string back into an integer. The method unwrap is used here to handle the potential error elegantly.

*/

//Changing Case
/*
Changing the case of strings is a common requirement in text processing.
Rust provides methods like to_lowercase and to_uppercase for this purpose. 
Using these methods does not transfer ownership.

fn main() {
    let s = String::from("Hello, WORLD!");
    let lower_s = s.to_lowercase();
    println!("{}", lower_s);  // Prints: "hello, world!"

    let upper_s = lower_s.to_uppercase();
    println!("{}", upper_s);  // Prints: "HELLO, WORLD!"

    println!("s still has ownership of {}", s); // Prints: "s still has ownership of Hello, WORLD!"
}

*/

//Trimming Whitespace

/*
Often, input strings might have extra whitespace that needs to be removed. 
Rust’s trim method is handy for these situations. 
Similar to the previous methods, trim does not transfer ownership.

fn main() {
    let s = String::from("   Hello, world!   ");
    let trimmed_s = s.trim();
    println!("'{}'", trimmed_s);  // Prints: "'Hello, world!'"
}
*/


//Handling Escape Characters

/*
Escape characters are used to represent special characters within strings. 
For example, to instruct Rust to treat quotes as part of the string instead of the closing quote, just prepend \ before the quotes. 
Rust provides functionality to handle these characters efficiently. To include a \ as part of the string, prepend \ with another \. To create a new line, use \n.


fn main() {
    let s = "Cosmo says \"hi\"";
    println!("{}", s);  // Prints: Cosmo says "hi"

    let s_escaped = "Hello\\world!";
    println!("{}", s_escaped);  // Prints: "Hello\world!"

    let s_new_line = "Hello\nWorld!";
    println!("{}", s_new_line); // Prints: Hello
                               //  World!
}

The \"hi\" allows quotes to be part of the string
The \\ escape character is used to include a backslash in the string.
The \n escape character is used to insert a newline in the string


*/


//Practice

/*
fn main() {
    let hero = "Iron Dude";
    let hero_name = hero.to_string(); 
    println!("Superhero as String: {}", hero_name);  // Prints: "Superhero as String: Iron Dude"

    let shout = String::from("Heroes, assemble!");
    let command = shout.as_str();
    println!("Command as string literal: {}", command);  // Prints: "Command as string literal: Heroes, assemble!"

    let power = 100;
    let power_str = power.to_string();
    println!("Power Level as String: {}", power_str);  // Prints: "Power Level as String: 100"

    let parsed_power: i32 = power_str.parse().unwrap();
    println!("Power Level as int: {}", parsed_power);  // Prints: "Power Level as int: 100"
}
*/

/*
fn main() {
    let hero = "Iron Man";
    let hero_name = hero.to_string();
    println!("Superhero as String: {}", hero_name);

    let shout = String::from("Avengers, assemble!");
    let command = shout.as_str();
    println!("Command as string literal: {}", command);  

    let power = "100";

    let parsed_power: i32 = power.parse().unwrap();
    println!("Power Level as int: {}", parsed_power);
}
*/

/*
fn main() {
    let message = String::from("WELCOME BACK, HERO!");
    // TODO: Convert the string `message` to all lowercase
    let lower_message = message.to_lowercase();
    println!("{}", lower_message);  // Expected: "welcome back, hero!"
    
    let greeting = String::from("welcome to the team!");
    // TODO: Convert the string `greeting` to all uppercase
    let upper_greeting = greeting.to_uppercase();
    println!("{}", upper_greeting);  // Expected: "WELCOME TO THE TEAM!"
    
    let announcement = String::from("   New mission assigned!   ");
    // TODO: Remove all whitespace from the string 
    let trimmed_announcement = announcement.trim()
    println!("'{}'", trimmed_announcement);  // Expected: "'New mission assigned!'"
}
*/


/*
fn main() {
    let motto = "Heroe's motto: \"Save the day!\"";
    println!("{}", motto);  // Expected: Heroe's motto: "Save the day!"
    

    let file_path = "~\\Hard_Drive\\Secrets\\x.txt";
    println!("{}", file_path);  // Expected: "~\Hard_Drive\Secrets\x.txt"
    
    let power1 = "Flight";
    let power2 = "Invisibility";
    println!("Power 1: {}\nPower 2:{}", power1, power2);
    //Expected
    // Power 1: Flight
    // Power 2: Invisibility
}
*/

/*
fn main() {
    let message = "\"Encryption Protocol\"";
    println!("Initiating {}", message);
    println!("Searching for file in \"~\\Hard_Drive\\Encrypted\\secrets.txt\"");
    // Expected Output:
    // Initiating "Encryption Protocol"
    // Searching for file in "~\Hard_Drive\Encrypted\secrets.txt"
}
*/

