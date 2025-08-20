/*
vectorsstore a collection of elements of the SAME TYPE.
However, UNLIKE arrays, vectors are DYNAMIC and can GROW and SHRINK as needed.
*/

// Creating Vectors

/*
Vectors can be created in Rust with or without specifying the data type explicitly. 
If the type is not explicitly mentioned, Rust will infer it based on the values pushed into the vector. 
To declare a new vector explicitly use Vec followed by the data type within <>. 
To add new elements to a vector, use push to append the new value to the end of the vector. 
To implicitly declare a vector, use vec! followed by the elements inside brackets.
*/

/*
fn main() {
    // Creating Vector with Data Type
    let mut vector_with_type: Vec<i32> = Vec::new();
    vector_with_type.push(1);
    vector_with_type.push(2);
    vector_with_type.push(3);

    // Creating Vector without Data Type (type inference)
    let mut vector_without_type = vec![4, 5, 6];
    vector_without_type.push(7);
}
*/

//Accessing Elements of a Vector

/*
You can access elements of a vector using both the get method and direct indexing. 
The get method returns an Option type that can be used to handle out-of-bounds errors gracefully. 
The get method returns an Option<&T> where T is the type of the elements in the vector. 
The Option type can be Some(&element) if the index is valid, or None if the index is out of bounds.
To ensure the valid access of an element, use the pattern matching construct if let Some(&element) = vector.get(index). 
If index is indeed a valid index, element takes on the value of the element in the vector, and the if block is executed. 
If index is not a valid index, element takes on the value of None, and the if block does not execute.
*/

/*
fn main() {
    let vector = vec![1, 2, 3];
    if let Some(first_elem) = vector.get(0) {
        println!("First element (using get): {:?}", first_elem); // Prints: First element (using get): 1
    }
    println!("Second element (using index): {}", vector[1]); // Prints: Second element (using index): 2
}
*/

//Modifying Vectors

/*
Vectors in Rust are immutable by default, but can be made mutable using the mut keyword. 
This allows you to modify their elements and change their size
*/

/*
fn main() {
    let mut mutable_vector = vec![8, 9, 10];
    mutable_vector[1] = 42; // Modifying the second element
    println!("Updated vector: {:?}", mutable_vector); // Prints: Updated vector: [8, 42, 10]
}
*/


//Removing Elements from a Vector

/*
Rust provides methods like pop and remove to remove elements from a vector, simplifying management tasks.
.pop returns the last element of the vector and removes it from the vector.

The remove method is used to remove an element from a vector at a specified index. 
This operation shifts all elements after the specified index one position to the left, 
effectively reducing the vector's length by one. The method also returns the removed element.
*/

/*
fn main() {
    let mut vector = vec![4, 5, 6, 7];
    println!("Popped element: {:?}", vector.pop()); // Prints: Popped element: Some(7)
    let removed_elem = vector.remove(1); // Removes the element at index 1
    println!("Removed Element: {}", removed_elem); // Prints: Removed Element: 5
    println!("Updated vector: {:?}", vector); // Prints: Updated vector: [4, 6]
}
*/

/*
vector.pop() removes the last element and returns it wrapped in Some, or None if the vector is empty.
vector.remove(1) removes and returns the element at index 1. 
If the index is out-of-bounds, this code will cause an error during runtime.
*/

//Ownership and Copy Data in Vectors

/*
Handling ownership and the concept of copy versus non-copy data types is crucial in Rust. 
Vectors themselves are not Copy types in Rust. 
Even if the elements inside the vector are of a type that implements the Copy trait (such as i32), the vector itself does not implement the Copy trait. 
Like arrays, using direct indexing on a vector (vector[index]) does not move ownership of a non-copy element. 
You can only create a reference to the element. Here's how it works with vectors
*/

/*
fn main() {
    let vector_with_copy = vec![1, 2, 3, 4];
    
    // Copy data
    let first_elem_copy = vector_with_copy[0];
    println!("first_elem_copy: {}", first_elem_copy); // Prints: first_elem_copy: 1
    println!("vector_with_copy: {:?}", vector_with_copy); // Prints: vector_with_copy: [1, 2, 3, 4]
    
    // Non-copy data
    let vector_with_non_copy = vec![String::from("Hello"), String::from("World")];
    
    let first_elem_non_copy = &vector_with_non_copy[0]; // Creates a reference
    println!("first_elem_non_copy: {}", first_elem_non_copy); // Prints: first_elem_non_copy: Hello
    let invalid_copy = vector_with_non_copy[0]; // Causes an error. You cannot move ownership of vector elements

    // Ownership Transfer
    let copied_vector = vector_with_copy; // Ownership moved to copied_vector
    println!("vector_with_copy: {:?}", vector_with_copy); // Causes an error
}
*/

//Vector with Copy Data

/*
vector_with_copy contains i32 elements, which are copy types.
vector_with_copy[0] accesses the first element, 1, which is of type i32.
first_elem_copy is assigned the value 1. Since i32 implements the Copy trait, the value is copied rather than moved.
*/

//Vector with Non-Copy Data

/*
vector_with_non_copy is initialized with String elements. String does not implement the Copy trait.
The line &vector_with_non_copy[0] creates a reference to the first element of the vector
The line vector_with_non_copy[0] causes an error because you cannot move ownership of elements in a non-Copy vector
*/

//Ownership Transfer

/*
Ownership of vector_with_copy is moved to copied_vector, making vector_with_copy invalid.
*/

//Slicing Vectors

/*
Slices allow you to work with portions of a vector without creating a new one. 
To create a vector slice, use a reference to the vector followed the starting index up to, but not including the ending index. 
To create a full slice of an vector, simply place .. inside the brackets. 
Here’s how you can create and use slices
*/

/*
fn main() {
    let vector = vec![10, 20, 30, 40];
    let slice = &vector[1..3]; // Slicing the vector
    println!("Slice from vector: {:?}", slice); // Prints: Slice from vector: [20, 30]

    let mut vector_for_slice = vec![10, 20, 30, 40];
    {
        let mutable_slice = &mut vector_for_slice[1..3];
        mutable_slice[0] = 25; // Modifying the slice
        mutable_slice[1] = 35; // Modifying the slice
    }
    println!("Vector after modifying slice: {:?}", vector_for_slice); // Prints: Vector after modifying slice: [10, 25, 35, 40]
}
*/


//Vectors as Function Parameters

/*
Vectors can be passed to functions by reference or by value, affecting ownership and data access. 
The rules for passing vectors to functions are as follows

Passing a reference to an vector does not transfer ownership.
Unlike arrays, passing a vector by value always transfers ownership, regardless of whether the elements implement the Copy trait or not.
*/


/*
fn main() {
    let vector1 = vec![String::from("Hello"), String::from("World")];
    display_vector_reference(&vector1);
    println!("After display_vector_reference: {:?}", vector1); // Prints: After display_vector_reference: ["Hello", "World"]

    let vector2 = vec![1, 2, 3, 4];
    display_vector_ownership(vector2);
    println!("After display_vector_ownership: {:?}", vector2); // Error as ownership has been moved
}

fn display_vector_reference(vec: &Vec<String>) {
    println!("In display_vector_reference: {:?}", vec); // Prints: In display_vector_reference: ["Hello", "World"]
}

fn display_vector_ownership(vec: Vec<i32>) {
    println!("In display_vector_ownership: {:?}", vec); // Prints: In display_vector_ownership: [1, 2, 3, 4]
}
*/

/*
fn main() {
    // Creating a new vector
    let mut v: Vec<i32> = Vec::new();
    // Adding values to the vector
    v.push(1);
    v.push(2);
    v.push(3);

    // Display the values in the vector
    println!("{:?}", v);

    // Creating a vector using the vec! macro
    let v2 = vec![1, 2, 3, 4];
    // Accessing an element in the vector using indexing
    let third: i32 = v2[2];
    println!("The third element is {}", third);
    
    // Access an element in the vector using get
    if let Some(fourth) = v2.get(3) {
        println!("The fourth element is {}", fourth);
    }
    
    // Modifying an element in the vector
    let mut v3 = vec![1, 2, 3, 4];
    v3[2] = 5;
    println!("Modified vector: {:?}", v3);
}
*/

/*
fn main() {
    let vector_with_copy = vec![1, 2, 3, 4];
    
    // Copy data
    let first_elem_copy = vector_with_copy[0];
    println!("first_elem_copy: {}", first_elem_copy); // Prints: first_elem_copy: 1
    println!("vector_with_copy: {:?}", vector_with_copy); // Prints: vector_with_copy: [1, 2, 3, 4]

    // Non-copy data
    let vector_with_non_copy = vec![String::from("Hello"), String::from("World")];
    let first_elem_non_copy = &vector_with_non_copy[0]; 
    println!("first_elem_non_copy: {}", first_elem_non_copy); // Prints: first_elem_non_copy: Hello

    // Ownership Transfer
    let copied_vector = vector_with_copy; // Ownership moved to copied_vector
    // println!("vector_with_copy: {:?}", vector_with_copy); Causes an error
}
*/

/*
fn main() {
    // Creating a vector with common superhero names
    let mut heroes = vec!["Superman", "Batman", "Wonder Woman"];

    // TODO: Print out the third superhero using indexing
    println!("The third hero is: {}", heroes[2]);

    // TODO: Change the second superhero to "Robin" and print it
    heroes[1] = "Robin";
    println!("The updated second hero is: {:?}", heroes);
}
*/

/*
fn main() {
    let superhero_stats = vec![String::from("Superman"), String::from("100")];
    let _name = &superhero_stats[0]; // Hint: Can we move ownership of non-copy data in vectors?
    let power_level = &superhero_stats[1]; // Hint: Can we move ownership of non-copy data in vectors?

    println!("Superhero name: {}", _name);
    println!("Power level: {}", power_level);
}
*/

/*
fn main() {
    // TODO: Create a mutable vector named `scores` and initialize it with elements [95, 82, 73]
    let mut scores = vec![95,82,73];
    // TODO: Remove the last element of scores and print its value
    println!("Last element is removed {:?}",scores.pop());
    // TODO: Add a new score of 78 to the vector
    scores.push(78);
    // TODO: Verify the new value was added to scores by printing out the last element
    println!("scores after adding new value {:?}",scores);
    // TODO: Modify the first element of `scores` to be 100
    scores[0] = 100;
    // TODO: Print the whole contents of the vector
    println!("Whole content of vector is {:?}", scores);
}
*/

/*
fn main() {
    let mut hero_names = vec![String::from("Iron Man"), String::from("Thor")];
    
    let mut power_levels = vec![2999, 4500];
    
    // TODO: Call add_hero without transferring owernship of either vectors
    add_hero(&mut hero_names, &mut power_levels);
    // TODO: Print updated hero_names
    println!("hero names are {:?}",hero_names);
    // TODO: Print updated power_levels
    println!("Power levels are {:?}",power_levels);
}

// TODO: Implement add_hero that adds a new name to hero_names and a new power level to power_levels
// Hint: This function should take in mutable references
fn add_hero(names: &mut Vec<String>, levels: &mut Vec<i32>) {
    println!("{:?} {:?}", names, levels);
}
*/

fn main() {
    let mut hero_names = vec![String::from("Iron Man"), String::from("Thor")];

    let mut power_levels = vec![2999, 4500];
    
    // TODO: Call add_hero without transferring owernship of either vectors
    add_hero(& mut hero_names, &mut power_levels);
    // TODO: Print updated hero_names
    println!("{:?}",hero_names);
    // TODO: Print updated power_levels
    println!("{:?}",power_levels);
}

// TODO: Implement add_hero that adds a new name to hero_names and a new power level to power_levels
// Hint: This function should take in mutable references
fn add_hero(names: &mut Vec<String>, levels:&mut Vec<i32>) {
names.push(String::from("Batman"));
levels.push(5000);
}