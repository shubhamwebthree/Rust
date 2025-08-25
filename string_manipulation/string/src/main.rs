//Strings

/* 
Strings are an essential part of any programming language because they enable you to store and manipulate text. 
In this lesson, we'll delve into the fundamental concepts of strings in Rust, covering literals, the String type, references, and string slices. 
By the end of this lesson, you'll have a solid understanding of how to work with strings efficiently and effectively in Rust.
*/

//String Literals
/*

String literals are the most basic form of strings in Rust. 
They are immutable and stored directly in the binary file that also holds the code. 

fn main() {
    let greeting = "Hello, world!";
    println!("{}", greeting); // Prints: Hello, world!
}
*/

//String Type
/*
The String type in Rust is more complex and flexible than string literals. 
It supports mutability and is allocated in memory when the program runs. 
To declare a String, use String::from followed by a string in quotes.

fn main() {
    let hello = String::from("Hello");
    println!("{}", hello); // Prints: Hello
}

The String type is useful when you need a growable, mutable text representation.
*/

//Strings, String Literals, and References
/*
difference between a String and string literal.


String literals are immutable and stored directly in the program's binary. 
They have a static lifetime, meaning they are valid for the entire duration of the program. 
Assigning a variable to a string literal creates an immutable reference with a static lifetime (&'static str). 
Therefore, string literals are not considered Copy types. Assigning a new variable to a string literal does not transfer ownership or make a copy of the data. 
Instead, the new variable is just a reference to where the string literal is stored in the code binary.

The String type is allocated in memory when the program runs. 
When you want to use a String without transfering ownership, use a reference instead by using &.


fn main() {
    // String Literal
    let s1 = "Hello";
    let s2 = s1;
    println!("{}", s1); // Prints: Hello
    println!("{}", s2); // Prints: Hello


    // String type
    let s3 = String::from("Hello");
    let s4 = &s3; // s4 is a reference to s3
    println!("I am a reference to {}", s4); // Prints: I am a reference to Hello
    println!("I still own {}", s3); // Prints: I still own Hello

}
*/

//String Slices

/*
String slices are references to parts of a string. 
They allow you to work with substrings without making a copy. 
To create a string slice from a String or string literal, use the syntax &var_name[start..end]. 
Just like arrays, Strings are zero indexed, and the upper bound of the range is exclusive. 


fn main() {
    // String Slices
    let s1 = String::from("Hello, world!");
    let hello = &s1[0..5];
    let world = &s1[7..12];
    println!("{} {}", hello, world); // Prints: Hello world

    let s2 = "Greetings Explorer!";
    let greeting = &s2[0..9]; 
    println!("{}", greeting); // Prints: Greetings
}
*/


//Practice

/*
fn main() {
    // String Literals
    let hero1 = "Spider-Man";
    println!("{}", hero1);
    let hero_ref = hero1;
    println!("{}", hero_ref);

    // String Type
    let hero2 = String::from("Iron Man");
    println!("{}", hero2);

    // References
    let hero3 = String::from("Thor");
    let ref_hero = &hero3; // ref_hero is a reference to hero3
    println!("{}", ref_hero);
    println!("{}", hero3); // hero3 can still be used here

    // String Slices
    let hero4 = String::from("Black Panther");
    let black = &hero4[0..5];
    let panther = &hero4[6..13];
    println!("{} {}", black, panther);

    let hero5 = "Captain America!";
    let captain = &hero5[0..7]; // Added a reference to make it a slice
    println!("{}", captain);
}
*/


fn main() {
    // TODO: Define a string literal "hero" with the value "Batman" and print it.
    let hero = "Batman";
    println!("Hero Literal {}", hero);
    // TODO: Define a String type "hero2" using String::from with the value "Wonder Woman" and print it.
    let hero2 = String::from("Wonder Woman");
    println!("String Type Hero {}",hero2);
    
    // TODO: Create a String "s1" with the value "Aquaman," create a reference "s2" to s1, and print both.
    let s1 = String::from("Aquaman");
    let s2 = &s1;
    
    println!("s1 is {}",s1);
    println!("s2 is also {}", s2);
    
    // TODO: Create a String "s1" with the value "Green Lantern," create slices "green" and "lantern" representing the first and second parts respectively, and print them.
    let s1 = String::from("Green lantern");
    let green = &s1[0..5];
    let lantern = &s1[6..13];
    
    println!("{}",green);
    println!("{}",lantern);
    
    // TODO: Create a string literal "s2" with the value "Flash Gordon!", create a slice "flash" from it, and print the slice.
    let s2 = "Flash Gordon!";
    let flash = &s2[0..5];
    println!("{}",flash);
}





