//Introduction to HashMaps in Rust


/*
we will focus on another powerful and versatile data structure in Rust's std::collections module HashMaps. 
HashMaps are invaluable when you need to establish a mapping between a set of keys and a corresponding set of values.

HashMaps store key-value pairs, making it easy to quickly look up values based on their associated keys. 
This concept is similar to dictionaries in other programming languages like Python. 
Let's dive in and get familiar with HashMaps!
*/

//Creating a HashMap

/*
In Rust, creating a HashMap involves using the HashMap struct from the std::collections module. 
When creating a new Hashmap, add the data type of the keys and data type of the values inside <>. 
You can also create an empty HashMap without specifying types, and Rust will infer the types based on how you insert an element into the HashMap for the first time.


use std::collections::HashMap;

fn main() {
    // Create a new HashMap with explicit types
    let mut hashmap: HashMap<&str, i32> = HashMap::new();

    // Create a new HashMap with inferred types
    let mut hashmap_inferred = HashMap::new();
}

*/

//Adding and Accessing Elements
 /*
 Once you have a HashMap, you can add elements using the insert method. 
 To access a value from a Hashmap use .get followed by the key name. 
 The .get method only accepts a reference.

use std::collections::HashMap;

fn main() {
    let mut hashmap: HashMap<&str, i32> = HashMap::new();

    // Add elements
    hashmap.insert("one", 1);
    hashmap.insert("two", 2);
    hashmap.insert("three", 3);

    // Access a value
    let value = hashmap.get("two");
    println!("Value under 'two': {:?}", value); // Prints: Value under 'two': Some(2)

    println!("{:?}", hashmap); // Prints: {"one": 1, "two": 2, "three": 3}
}
*/

//Modifying and Removing Elements
/*
HashMaps provide methods to modify values and remove elements by their keys. 
To remove an element, use .remove by passing in a variable reference. 
To modify the value stored in a key, you can reinsert the key with its new value or use .get_mut. .get_mut accepts a reference to the key.

use std::collections::HashMap;

fn main() {
    let mut hashmap: HashMap<&str, i32> = HashMap::new();
    hashmap.insert("one", 5);
    hashmap.insert("two", 2);
    hashmap.insert("three", 4);

    // Remove an element
    hashmap.remove("two");

    // Reinserting an element
    hashmap.insert("one", 1);

    // Modify an element
    if let Some(entry) = hashmap.get_mut("three") {
        *entry = 3;
    }
    
    println!("Updated HashMap: {:?}", hashmap); // Prints: Updated HashMap: {"one": 1, "three": 3}
}
*/

//Checking for Keys

/*
Checking for the existence of keys in a HashMap can be easily done using the contains_key method. 
The value passed to contains_key must be a reference.

use std::collections::HashMap;

fn main() {
    let mut squares: HashMap<i32, i32> = HashMap::new();
    squares.insert(1, 1);
    squares.insert(2, 4);

    // Check if a key exists
    let has_one = squares.contains_key(&1); // Passing a reference
    let has_three = squares.contains_key(&3); // Passing a reference
    println!("HashMap contains key 1: {}", has_one); // Prints: HashMap contains key 1: true
    println!("HashMap contains key 3: {}", has_three); // Prints: HashMap contains key 3: false
}
*/

//Understanding Ownership in HashMaps
/*
As with other data structures in Rust, managing ownership and borrowing is crucial when working with HashMaps. 
Elements added to a HashMap must adhere to Rust's ownership rules.

use std::collections::HashMap;

fn main() {
    // Create an empty HashMap
    let s = String::from("Hello");
    let mut map = HashMap::new();
    map.insert("key1", s); // Transfers ownership of "Hello"
    map.insert("key2", String::from("World"));
    println!("{}", s); // Causes an error
}
*/


//HashMaps as Function Parameters

/*
HashMaps can be passed to functions as references or by value. 
Understanding how to pass HashMaps to functions allows for more modular and reusable code. 
Similar to vectors and Hashsets, a HashMap is never copy type, even if the data held in the HashMap is copy type.

use std::collections::HashMap;

fn main() {
    let mut hashmap = HashMap::new();
    hashmap.insert("key1", 10);
    hashmap.insert("key2", 20);

    display_hashmap_reference(&hashmap);
    println!("After display_hashmap_reference: {:?}", hashmap); // Prints: After display_hashmap_reference: {"key1": 10, "key2": 20}

    display_hashmap_copy(hashmap); // Ownership moved to display_hashmap_copy
    // println!("After display_hashmap_copy: {:?}", hashmap); // Causes error
}

fn display_hashmap_reference(map: &HashMap<&str, i32>) {
    println!("In display_hashmap_reference: {:?}", map); // Prints: In display_hashmap_reference: {"key1": 10, "key2": 20}
}

fn display_hashmap_copy(map: HashMap<&str, i32>) {
    println!("In display_hashmap_copy: {:?}", map); // Prints: In display_hashmap_copy: {"key1": 10, "key2": 20}
}

*/

//Practice

/*
use std::collections::HashMap;

fn main() {
    // Create a new hashmap
    let mut superhero_powers: HashMap<&str, &str> = HashMap::new();

    // Add elements
    superhero_powers.insert("Superman", "Super Strength");
    superhero_powers.insert("Batman", "High Intelligence");
    superhero_powers.insert("Flash", "Super Speed");

    // Access values
    let power = superhero_powers.get("Batman");
    println!("Batman's power: {:?}", power);

    // Remove elements
    superhero_powers.remove("Batman");

    // Modify an element
    if let Some(entry) = superhero_powers.get_mut("Flash") {
        *entry = "Time Travel";
    }

    // Check if key exists
    let has_superman = superhero_powers.contains_key(&"Superman");
    let has_batman = superhero_powers.contains_key(&"Batman");
    println!("Contains Superman: {}, Contains Batman: {}", has_superman, has_batman);
}
    
*/



/*
use std::collections::HashMap;

fn main() {
    let mut hero_scores: HashMap<&str, i32> = HashMap::new();

    hero_scores.insert("Iron Man", 85);
    hero_scores.insert("Captain America", 90);

    // Change elements
    if let Some(entry) = hero_scores.get_mut("Captain America") {
        *entry = 95;
    }
    
    // Access values
    let score = hero_scores.get(&"Captain America");
    println!("Captain America's score: {:?}", score);
}
*/

/*
use std::collections::HashMap;

fn main() {
    // TODO: Create a mutable HashMap named `movie_ratings` that maps &str to i32
    let mut movie_ratings : HashMap<&str,i32> = HashMap::new();
    // TODO: Insert some movie titles with their ratings into the HashMap
    movie_ratings.insert(&"Iron Man",9);
    movie_ratings.insert(&"Batman",10);
    movie_ratings.insert(&"Thor",9);
    // TODO: Call the show_ratings function without transferring ownership
    show_ratings(&movie_ratings);
    // TODO: Print the HashMap to confirm ownership was not transferred
    println!("Ratings without ownership transfer {:?}",movie_ratings);
}

// TODO: Write a function show_ratings that prints out each movie and rating without taking ownership
fn show_ratings(map: &HashMap<&str,i32>){
    println!("{:?}",map);
}
*/

/*
use std::collections::HashMap;

fn main() {
    // Ownership for HashMap
    let book = String::from("Harry Potter");
    let mut book_map = HashMap::from([("book1", book), ("book2", String::from("Lord of the Rings"))]);
    // println!("{}", book); Causes an error since `book` transferred ownership to `book_map`

    // Hashmaps as function parameters
    let mut book_hashmap = HashMap::new();
    book_hashmap.insert("book3", "Percy Jackson");
    book_hashmap.insert("book4", "Game of Thrones");
    display_book_reference(&book_hashmap);
    println!("After display_book_reference: {:?}", book_hashmap);

    transfer_book_ownership(book_hashmap);
    // println!("After transfer_book_ownership: {:?}", book_hashmap); Causes error
}

fn display_book_reference(map: &HashMap<&str, &str>) {
    println!("Books in map: {:?}", map);
}

fn transfer_book_ownership(map: HashMap<&str, &str>) {
    println!("Transferred books: {:?}", map);
}
*/

/*
use std::collections::HashMap;

fn main() {
    // Create a new hashmap
    let mut hero_map: HashMap<&str, &str> = HashMap::new();
    // Add elements
    // TODO: Add "Thor" with the power "Thunder God"
    hero_map.insert("Thor","Thunder God");
    // TODO: Add "Iron Man" with the power "Genius Inventor"
    hero_map.insert("Iron Man", "Genius Inventor");
    // TODO: Add "Hulk" with the power "Strength"
    hero_map.insert("Hulk","Strength");
    
    // Access values
    // TODO: Access the power of "Iron Man" and store it in a variable
    let has_iron_man = hero_map.get(&"Iron Man");
    // TODO: Print the power of Iron Man using the variable
    println!("Power of Iron Man is {:?}",has_iron_man);
}
*/

/* 
use std::collections::HashMap;

fn main() {
    // TODO: Create a mutable HashMap named `movie_ratings` that maps &str to i32
    let mut movie_ratings : HashMap<&str,i32> = HashMap::new();
    // TODO: Insert some movie titles with their ratings into the HashMap
    movie_ratings.insert(&"Iron Man",9);
    movie_ratings.insert(&"Batman",10);
    movie_ratings.insert(&"Thor",9);
    // TODO: Call the show_ratings function without transferring ownership
    show_ratings(&movie_ratings);
    // TODO: Print the HashMap to confirm ownership was not transferred
    println!("Ratings without ownership transfer {:?}",movie_ratings);
}

// TODO: Write a function show_ratings that prints out each movie and rating without taking ownership
fn show_ratings(map: &HashMap<&str,i32>){
    println!("{:?}",map);
}
*/

