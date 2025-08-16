/*
Arrays are somewhat similar to tuples but come with their own unique set of characteristics and benefits
An array in Rust is a collection of elements of the same type, stored in a contiguous block of memory
This can be especially useful when you have a fixed-size collection of elements that you need to manage efficiently. 
Unlike tuples, every element of an array must be the same data type
*/


/*
There are two ways to declare arrays in Rust:
With explicit type and length: You specify both the data type of elements and the array's size. 
With type inference: You let Rust figure out both the data type and length based on the values you provide.
*/


// Example
/*
fn main() {
    // Creating Array with explicit Data Type and Length
    let array_with_type: [i32; 4] = [1, 2, 3, 4]; 
    // Creating Array without explicit Type or Length (compiler infers both)
    let array_without_type = [5, 6, 7, 8];

    println!("Array with type: {:?}", array_with_type); // Prints: [1, 2, 3, 4]
    println!("Array without type: {:?}", array_without_type); // Prints: [5, 6, 7, 8]
}
*/


/*
Accessing Elements of an Array
You can access the elements of an array using index notation
To access the value of an array, use the array name followed by square brackets containing the index number.
*/

// Example

/*
fn main() {
    let array = [1, 2, 3, 4];
    println!("First element: {}", array[0]); // Prints: First element: 1
    println!("Fourth element: {}", array[3]); // Prints: Fourth element: 4
}
*/

//Mutable Arrays

/*
In Rust, arrays are by default immutable. 
Using the mut keyword, we can modify the elements of an array. 
Keep in mind the data type of new values must be the same as the original value.
*/

/*
fn main() {
    let mut mutable_array = [9, 10, 11, 12];
    mutable_array[2] = 42; // Modifying the third element
    println!("Mutable array: {:?}", mutable_array); // Prints: Mutable array: [9, 10, 42, 12]
}
*/

//Copy and Non-copy Data in Arrays

/*
Understanding whether data in arrays can be copied or moved is crucial for effective Rust programming. 
If all the elements in an array implement the Copy trait, the array itself will also implement the Copy trait. 
Assigning an element of a non-copy Array to a variable is not allowed. 
Instead, you must use a reference.
*/

/*
fn main() {
    let array_with_copy = [1, 2, 3, 4]; // Array with Copy type data

    // Copy data
    let copy_array = array_with_copy; // Elements are copied
    println!("array_with_copy: {:?}", array_with_copy); // Prints: array_with_copy: [1, 2, 3, 4]
    println!("copy_array: {:?}", copy_array); // Prints: copy_array: [1, 2, 3, 4]

    // Non-copy data
    let array_with_non_copy = [String::from("Hello"), String::from("World")]; // Array with data that cannot be copied
    let first_elem = &array_with_non_copy[0]; // Creates a reference
    println!("Accessed element by reference: {}", first_elem); // Prints: Accessed element by reference: Hello
    let invalid_copy = array_with_non_copy[0]; // Causes an error. You cannot move ownership of array elements
    
    // Ownership Transfer
    let non_copy_array = array_with_non_copy; // Ownership moves
    println!("{:?}", array_with_non_copy); // Causes an error
}
*/


//Array Slices

/*
Slices in Rust provide a way to reference a contiguous sequence of elements from an array. 
They are particularly useful for working with subsections of an array without needing to create a new array. 
To create an array slice, use a reference to the array followed the starting index up to, but not including the ending index. 
To create a full slice of an array, simply place .. inside the brackets. 
*/

// example

/*
fn main() {
    let array = [1, 2, 3, 4];
    let slice = &array[1..3]; // Slicing the array
    println!("Slice from array: {:?}", slice); // Prints: Slice from array: [2, 3]

    let full_slice = &array[..]; // Full slice of the array
    println!("Full slice of array: {:?}", full_slice); // Prints: Full slice of array: [1, 2, 3, 4]

    // Modifying slice elements through a mutable slice
    let mut array_for_slice = [10, 20, 30, 40];
    {
        let slice = &mut array_for_slice[1..3];
        slice[0] = 25; // Modifying the slice
        slice[1] = 35; // Modifying the slice
    }
    println!("Array after modifying slice: {:?}", array_for_slice); // Prints: Array after modifying slice: [10, 25, 35, 40]
}
*/

// Arrays as Function Parameters
/*
Arrays can be passed to functions in Rust, making it possible to work with fixed-size collections efficiently. 
Similar to tuples, arrays can be passed by reference or by value, depending on whether you want to transfer ownership or simply allow the function to read the data. 
The rules for passing arrays as function parameters are:

Passing a reference to an array does not transfer ownership.
Passing an array by value copies the array if its elements implement the Copy trait.
Arrays composed of non-copy elements transfer ownership if passed by value.
*/

//Example
/*
fn main() {
    let array_with_non_copy = [String::from("Hello"), String::from("World")];
    display_array_reference(&array_with_non_copy);
    println!("After display_array_reference: {:?}", array_with_non_copy); // Prints: After display_array_reference: ["Hello", "World"]

    let array_with_copy = [1, 2, 3, 4];
    display_array_copy(array_with_copy);
    println!("After display_array_copy: {:?}", array_with_copy); // Prints: After display_array_copy: [1, 2, 3, 4]

    display_array_ownership(array_with_non_copy);
    // println!("After display_array_ownership: {:?}", array_with_non_copy); // Causes error
}

fn display_array_reference(arr: &[String; 2]) {
    println!("In display_array_reference: {:?}", arr); // Prints: In display_array_reference: ["Hello", "World"]
}

fn display_array_copy(arr: [i32; 4]) {
    println!("In display_array_copy: {:?}", arr); // Prints: In display_array_copy: [1, 2, 3, 4]
}

fn display_array_ownership(arr: [String; 2]) {
    println!("In display_array_ownership: {:?}", arr); // Prints: In display_array_ownership: ["Hello", "World"]
}
*/


/*
fn main() {
    let hero_names = [String::from("Superman"), String::from("Batman")];
    let hero_powers: [i32; 2] = [200, 100];

    // Passing a reference to array
    display_hero_names(&hero_names);
    println!("After display_hero_names: {:?}", hero_names); // No ownership moved, prints same array

    // Passing an array of copy types
    display_hero_powers(hero_powers);
    println!("After display_hero_powers: {:?}", hero_powers); // Data copied, prints same array
    
    // Passing an array of non-copy types
    transfer_hero_names(hero_names);
    // println!("After transfer_hero_names: {:?}", hero_names); // Causes error because ownership has been transfered
}

fn display_hero_names(names: &[String; 2]) {
    println!("Inside display_hero_names: {:?}", names);
}

fn display_hero_powers(powers: [i32; 2]) {
    println!("Inside display_hero_powers: {:?}", powers);
}

fn transfer_hero_names(names: [String; 2]) {
    println!("Inside transfer_hero_namess: {:?}", names);
}
*/


/*
fn main() {
    let mut power_levels = [25, 99]; 
    println!("Before increasing power level: {:?}", power_levels);
    {
    // TODO: Decrease the 1st power level by 5
        power_levels[0] -=5;
    // TODO: Increase the 2nd power level by 1   
    power_levels[1] +=1;
}
println!("After modification: {:?}", power_levels);
}
*/

fn main() {
    let hero_names = [String::from("Iron Man"), String::from("Thor")];
    let mut hero_powers = [2999, 4500];

    // Borrow, no ownership moved
    list_names(&hero_names);

    // Move ownership of names + mutate powers
    move_teams(hero_names, &mut hero_powers);

    println!("Hero powers are now: {:?}", hero_powers);
}

// Borrow, no transfer
fn list_names(list: &[String; 2]) {
    println!("Hero names are: {:?}", list);
}

// Move names (ownership), mutate powers
fn move_teams(names: [String; 2], powers: &mut [i32; 2]) {
    println!("Moving {:?} to another team", names);

    for power in powers.iter_mut() {
        *power = 0;
    }
}
