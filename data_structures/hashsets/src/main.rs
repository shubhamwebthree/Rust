/*
HashSets is  a powerful data structure in Rust that belongs to the collections module.
HashSets provide us with an efficient way to store and manage unique items. 
HashSet is an unordered collection that uses a hash function to manage its elements, 
ensuring that each element is unique.
This makes HashSets incredibly useful for tasks where you need to check for membership, eliminate duplicates, or perform set operations.
*/

//Creating a HashSet

/*
HashSet involves using the HashSet struct from the std::collections module. 
You can either create an empty HashSet and then add elements to it or create a Hashset with default values
*/

/*
use std::collections::HashSet;

fn main() {
    // Create an empty HashSet
    let mut empty_set: HashSet<i32> = HashSet::new();
    let mut set = HashSet::from([1,2,3,4]);
}
*/

/*
We first import the HashSet struct from the std::collections module.
We then create an empty HashSet named empty_set, which can store i32 values.
We then create a HashSet named set, which already contains some values.
*/


//Adding and Removing Elements
//Once you have a HashSet, you can add or remove elements using the insert and remove methods.

/*
use std::collections::HashSet;
fn main() {
    let mut hashset: HashSet<i32> = HashSet::new();

    // Add values to HashSet
    hashset.insert(1);
    hashset.insert(2);
    hashset.insert(3);

    // Remove values from HashSet
    hashset.remove(&2);
}
*/

/*
The insert method adds a value to the HashSet. 
If the value already exists, it will not be added again.
The remove method removes a value from the HashSet, if it exists. 
The value passed into remove must always be a reference.
*/


//Checking Membership and Other Properties
/*
One of the key advantages of using a HashSet is the ability to quickly check if an item exists within the set. 
You can also check the length of the HashSet and whether it's empty.
*/

/*
use std::collections::HashSet;
fn main() {
    let mut hashset: HashSet<i32> = HashSet::new();
    
    hashset.insert(1);
    hashset.insert(3);

    // Check membership in HashSet
    let has_one = hashset.contains(&1);
    let has_two = hashset.contains(&2);
    println!("HashSet has 1: {}, has 2: {}", has_one, has_two); // Prints: "HashSet has 1: true, has 2: false"

    // len() - get the number of elements
    let length = hashset.len();
    println!("Length of HashSet: {}", length); // Prints: "Length of HashSet: 2"

    // is_empty() - check if the set is empty
    let is_empty = hashset.is_empty();
    println!("Is HashSet empty: {}", is_empty); // Prints: "Is HashSet empty: false"
}
*/

/*
The contains method checks whether a value exists in the HashSet and returns a boolean. 
contains always expects a reference as an input.
The len method returns the number of elements in the HashSet.
The is_empty method checks if the HashSet is empty.
*/

//Understanding Ownership in HashSets

/*
As with other data structures in Rust, managing ownership and borrowing is crucial when working with HashSets. 
Elements added to a HashSet must adhere to Rust's ownership rules
*/

/*
use std::collections::HashSet;
fn main() {
    let s = String::from("Hello");
    let mut set = HashSet::from([s, String::from("World")]);
    println!("{}", s); // Causes an error
}
*/

//HashSets as Function Parameters

/*
HashSets can be passed to functions as references or by value. 
Understanding how to pass HashSets to functions allows for more modular and reusable code. 
Unlike tuples and arrays, a HashSet is never copy type, even if the data held in the HashSet is copy type.
*/  


/*
use std::collections::HashSet;
fn main() {
    let mut hashset = HashSet::new();
    hashset.insert(10);
    hashset.insert(20);

    display_hashset_reference(&hashset);
    println!("After display_hashset_reference: {:?}", hashset); // Prints: "After display_hashset_reference: {10, 20}"

    display_hashset_ownership(hashset); // Ownership moved to display_hashset_ownership
    println!("After display_hashset_ownership: {:?}", hashset); // Causes error
}

fn display_hashset_reference(set: &HashSet<i32>) {
    println!("In display_hashset_reference: {:?}", set); // Prints: "In display_hashset_reference: {10, 20}"
}

fn display_hashset_ownership(set: HashSet<i32>) {
    println!("In display_hashset_ownership: {:?}", set); // Prints: "In display_hashset_ownership: {10, 20}"
}
*/

/*
display_hashset_reference takes a reference to a HashSet, so it doesn't take ownership, allowing the HashSet to remain available after the function call.
display_hashset_ownership takes a HashSet by value. 
Even though the elements are a copyable data type (i32), a HashSet is not copy type, thus ownership is transferred.
*/


//Practice

/*
use std::collections::HashSet;

fn main() {
    // Create new HashSet
    let mut superhero_set: HashSet<&str> = HashSet::new();

    // Add superheroes to HashSet
    superhero_set.insert("Superman");
    superhero_set.insert("Batman");
    superhero_set.insert("Wonder Woman");

    // Remove a superhero from HashSet
    superhero_set.remove("Batman");

    // Check membership in HashSet
    let has_superman = superhero_set.contains("Superman");
    let has_batman = superhero_set.contains("Batman");
    println!("HashSet has Superman: {}, has Batman: {}", has_superman, has_batman);

    // len(), is_empty()
    println!("Length of HashSet: {}", superhero_set.len());
    println!("Is HashSet empty: {}", superhero_set.is_empty());
}
*/

/*
use std::collections::HashSet;

fn main() {
    // Ownership for HashSet
    let movie = String::from("Justice League");
    let mut _movie_set = HashSet::from([movie, String::from("Aquaman")]);
    // println!("{}", movie); Causes an error since `movie` transferred ownership to `movie_set`

    // Hashsets as function parameters
    let mut movie_hashset = HashSet::new();
    movie_hashset.insert("Shazam");
    movie_hashset.insert("Batman v Superman");
    display_movie_reference(&movie_hashset);
    println!("After display_movie_reference: {:?}", movie_hashset);

    transfer_movie_ownership(movie_hashset); 
    // println!("After transfer_movie_ownership: {:?}", movie_hashset); Causes error
}

fn display_movie_reference(set: &HashSet<&str>) {
    println!("Movies in set: {:?}", set);
}

fn transfer_movie_ownership(set: HashSet<&str>) {
    println!("Transferred movies: {:?}", set);
}
*/

/*
use std::collections::HashSet;

fn main() {
    // Create new HashSet
    let mut student_ids: HashSet<i32> = HashSet::new();

    // Add values to HashSet
    student_ids.insert(1); 
    student_ids.insert(2); 
    student_ids.insert(3); 

    // Remove values from HashSet
    student_ids.remove(&2);

    // Check membership in HashSet
    let has_id_1 = student_ids.contains(&1);
    let has_id_2 = student_ids.contains(&2);
    println!("Student 1 present: {}. Student 2 present: {}", has_id_1, has_id_2);
}
*/

/*
use std::collections::HashSet;

fn main() {
    // Create new HashSet
    let mut superhero_set: HashSet<&str> = HashSet::new();

    // TODO: Add values "Iron Man," "Thor," and "Black Widow" to the HashSet
    superhero_set.insert("Iron Man");
    superhero_set.insert("Thor");
    superhero_set.insert("Black Widow");
    // TODO: Remove "Thor" from the HashSet
    superhero_set.remove("Thor");
    // TODO: Check for membership of "Iron Man" and "Thor" and print the results
    let has_iron_man = superhero_set.contains("Iron Man");
    let has_thor = superhero_set.contains("Thor");
    println!("has Iron Man : {} & has Thor : {}", has_iron_man, has_thor );
    // TODO: Print the length of the HashSet
    let length_hashset = superhero_set.len();
    println!("{:?}", length_hashset);
    // TODO: Check if the HashSet is empty and print the result
    if length_hashset > 0 {
        println!("Hashset is not Emply");
    }
}
*/

/*
use std::collections::HashSet;

fn main() {
    // TODO: Create a new HashSet for city names
    let mut city_names : HashSet<&str> = HashSet :: new();
    // TODO: Add "Gotham", "Metropolis", and "Central City" to the HashSet
    city_names.insert("Gotham");
    city_names.insert("Metropolis");
    city_names.insert("Central City");

    // TODO: Call the display_cities function without transferring ownership
    display_cities(&city_names);
    // TODO: Print out the cities to confirm ownership was not transferred
        println!("{:?}", city_names);

}

// TODO: Write a function display_cities that prints out each city without taking ownership
fn display_cities(city_name:&HashSet<&str>){
    println!("{:?}", city_name);
}
*/
