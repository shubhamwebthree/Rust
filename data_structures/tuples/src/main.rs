/*
Tuples in Rust let us group together values of different data types into a single compound. 
They are especially useful when you want to work with heterogeneous data without relying on complex data structures.
Imagine you need to store a person's name, age, and height together. 
A tuple is an ideal candidate for this scenario.
*/

/*
There are two ways to declare tuples in Rust. 
To declare a tuple with explicit data types, add the data type in parentheses, separated by commas. 
The values for each element in the tuple must match these explicitly declared data types. 
Rust can also infer the data types, eliminating the need to explicitly declare data types.
*/

/*
example

fn main() {
    // Creating Tuple with Data Type
    let person_with_type: (&str, i32, f64) = ("Alice", 30, 5.5);

    // Creating Tuple without Data Type
    let person_without_type = ("Bob", 25, 6.0);
}
*/

// Accessing Elements of a Tuple

/*
fn main() {
    let person = ("Alice", 30, 5.5);
    // Accessing elements using index notation
    let name = person.0;
    let age = person.1;
    let height = person.2;
    println!("Name: {}", name);  // Prints: Name: Alice
    println!("Age: {}", age);    // Prints: Age: 30
    println!("Height: {}", height); // Prints: Height: 5.5
}
*/

// Deconstructing Tuples

/*
fn main() {
    let person = ("Alice", 30, 5.5);
    let (name, age, height) = person;
    println!("Name: {}", name);  // Prints: Name: Alice
    println!("Age: {}", age);    // Prints: Age: 30
    println!("Height: {}", height); // Prints: Height: 5.5
}
*/


// Mutable Tuples

/*
fn main() {
    let mut person = ("Charlie", 28, 5.8);
    println!("Before modification: {:?}", person); // Prints: ("Charlie", 28, 5.8)
    person.1 = 29; // Modifying the age
    println!("After modification: {:?}", person); // Prints: ("Charlie", 29, 5.8)
}
*/

// Ownership with Tuples

/*
fn main() {
    let tuple_with_copy = (42, "Hello".to_string());
    let (num, text) = tuple_with_copy;
    println!("Copy data in tuple - num: {}", tuple_with_copy.0); // Prints: "Copy data in tuple - num: 42"
    println!("Moved data is {}", tuple_with_copy.1); // Ownership of "Hello" has changed, so this line causes an error.
}
*/

// We created a tuple with an integer (i32) and a String.
// When deconstructing, num (i32) is copied because integers implement the Copy trait.


//Tuples as Function Parameters

/*
Tuples can be passed to functions, enabling us to bundle multiple elements as a single argument. 
If all elements within a tuple implement the Copy trait, the tuple itself can be copied

Rules for passing tuples as function parameters :-

1)Passing a reference to a tuple does not transfer ownership.
2)If all elements within a tuple implement the Copy trait, 
the tuple will not transfer ownership when passed to a function without using a reference.
3)If at least one element in the tuple is a non-copy type, 
ownership is transferred when the tuple is passed to a function without using a reference.





*/

/*
fn main() {
    let non_copy_tuple = (10, String::from("I am not copy"));
    display_tuple_reference(&non_copy_tuple);
    println!("After display_tuple_reference: ({}, {})", non_copy_tuple.0, non_copy_tuple.1); 
    // Prints: After display_tuple_reference: (10, I am not copy)
    
    let copyable_tuple = (10, 20);
    display_tuple_copy(copyable_tuple);
    println!("After display_tuple_copy: ({}, {})", copyable_tuple.0, copyable_tuple.1); 
    // Prints: After display_tuple_copy: (10, 20)
    
    display_tuple_ownership(non_copy_tuple);
    // println!("After display_tuple_ownership: ({}, {})", non_copy_tuple.0, non_copy_tuple.1); // Causes error
}

fn display_tuple_reference(tuple: &(i32, String)) {
    println!("In display_tuple_reference: ({}, {})", tuple.0, tuple.1); 
    // Prints: In display_tuple_reference: (10, I am not copy)
}

fn display_tuple_copy(tuple: (i32, i32)) {
    println!("In display_tuple_copy: ({}, {})", tuple.0, tuple.1); 
    // Prints: In display_tuple_copy: (10, 20)
}

fn display_tuple_ownership(tuple: (i32, String)) {
    println!("In display_tuple_ownership: ({}, {})", tuple.0, tuple.1); 
    // Prints: In display_tuple_ownership: (10, I am not copy)
}

*/


/*
1)The function display_tuple_reference takes a reference to a tuple with elements (i32, String). 
Referencing allows the function to read the tuple without taking ownership of its data, 
so it remains available in the main function.
2)The function display_tuple_copy takes a tuple with two i32 elements. 
Since i32 implements the Copy trait, the data is copied when passed to the function.
3)The function display_tuple_ownership takes ownership of a tuple with elements (i32, String). 
This consumes the tuple, making it unavailable for further use in the main function. 
Uncommenting the last println! statement in main causes an error because the tuple's ownership has been moved to the function.
*/

/*
Tuples are a versatile and powerful feature in Rust, 
offering an efficient way to group multiple values, 
adding an extra dimension of functionality to your programming toolkit.
*/


//Practice

/*
fn main() {
    // Creating Tuple with Data Type
    let person_with_type: (&str, i32, f64) = ("Alice", 30, 5.5);
    // Creating Tuple without Data Type
    let person_without_type = ("Bob", 25, 6.0);
    // Accessing elements directly using dot notation
    println!("{}", person_with_type.0);
    println!("{}", person_without_type.2);
    // Deconstructing Tuples
    let (name, age, height) = person_with_type;
    println!("Name: {}, Age: {}, Height: {}", name, age, height);

    // Mutable Tuples
    let mut mutable_tuple = ("Charlie", 28, 5.8);
    println!("Before modification: {:?}", mutable_tuple);
    mutable_tuple.1 = 29; // Modifying the age
    println!("After modification: {:?}", mutable_tuple);
}
*/


/*
fn main() {
    let movie_ratings = ("Inception", 9.1, 132);
    println!("Initial movie ratings: {:?}", movie_ratings);

    update_rating(&movie_ratings);
    println!("After update_rating: {:?}", movie_ratings);  // Should remain unchanged ("Inception", 9.1, 132)

    let mut mutable_movie_ratings = ("The Matrix", 8.7, 136);
    modify_rating(&mut mutable_movie_ratings);
    println!("After modify_rating: {:?}", mutable_movie_ratings);  // Should reflect changes ("The Matrix", 9.7, 146)

    let ratings = (7.5, 9.3);
    compare_ratings(ratings);
    println!("After compare_ratings: {:?}", ratings);  // Should remain unchanged (7.5, 9.3)

    let non_copy_tuple = (8.4, String::from("Batman"));
    change_non_copy_tuple(non_copy_tuple);
    // println!("After change_non_copy_tuple: {:?}", non_copy_tuple);  // Uncommenting this will throw an error due to ownership
}

fn update_rating(tuple: &(&str, f64, i32)) {
    println!("Rating before: ({}, {}, {})", tuple.0, tuple.1, tuple.2);
}

// Passing a mutable reference
fn modify_rating(tuple: &mut (&str, f64, i32)) {
    tuple.1 += 1.0;
    tuple.2 += 10;
}

fn compare_ratings(tuple: (f64, f64)) {
    println!("Comparing ratings: ({}, {})", tuple.0, tuple.1);
}

fn change_non_copy_tuple(tuple: (f64, String)) {
    println!("Changing non-copy tuple: ({}, {})", tuple.0, tuple.1);
}
*/


fn main() {
    
    let mut smartphone = (String::from("iPhone"), 12, 799.99);
    let dimensions = (1920,1080);
    
    // TODO: Call describe_smartphone WITHOUT transfering ownership
    describe_smartphone(&smartphone); // Expected output: Phone: iPhone, Model: 12, Price: $799.99
    
    // TODO: Call update_version WITHOUT transfering ownership
    update_version(&mut smartphone);
    println!("New phone version: {}", smartphone.1); // Expected output: New phone version: 13
    
    // TODO: Call validate_dimensions WITHOUT passing a reference
    validate_dimensions(dimensions); // Expected output: Dimensions validated.
    println!("Dimensions: {:?}", dimensions); // Expected output: Dimensions: (1920, 1080)
    
    // TODO: Call sell_smartphone in order to transfer ownership
    sell_smartphone(smartphone); // Expected output: Selling iPhone 13 for $799.99.
}

// TODO: Implement describe_smartphone function that prints the phone's details without transfering ownership
fn describe_smartphone(tuple:&(String,i32,f32)) {
    println!("{} {} {}",tuple.0, tuple.1,tuple.2);
}

// TODO: Implement update_version which increases the version by 1 without transfering ownership.
fn update_version(tuple:&mut(String,i32,f32)) {
tuple.1 += 1;
}

// TODO: Implement validate_dimensions function that simply prints "Dimensions validated" This function DOES NOT accept a reference as a parameter
fn validate_dimensions(tuple:(i32,i32)) {
    // TODO: If both dimensions are greater than 0, print "Dimensions validated"
    if tuple.0 > 0 && tuple.1 > 0 {
        println!("dimensions validated");
    }else{
    // TODO: Else, print "Invalid dimensions"
        println!("invalid dimensions");
    }
}

// TODO: Implement sell_smartphone function that transfers ownership of the phone and prints out it's details
fn sell_smartphone(tuple: (String,i32,f32)) {
println!("{} {} {}", tuple.0, tuple.1, tuple.2);
}