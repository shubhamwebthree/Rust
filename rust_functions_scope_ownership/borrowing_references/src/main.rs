/*
Borrowing Immutable References: How to access variables without transferring ownership.
Borrowing Mutable References: How to modify data through references without ownership changes.
Rules of Mutable References: The constraints Rust imposes to ensure safe modifications.
Dangling References: How Rust prevents pointers to non-existent objects and ensures memory safety.
*/

/*

Borrowing Immutable References (in this case s: &String). 

Rust defaults to transferring a variable's ownership when it's passed to a function. 
If we want to pass a variable to a function without transferring ownership, 
Rust uses the borrowing mechanism. 
To do this, the function signature declares its input as a reference 
(in this case s: &String). 
To call the function, 
pass a reference to a variable (in this case &title).


fn main() {
    let title = String::from("Rust Programming");  
    // Here, we declare a new String variable title
    borrow_reference(&title);  
    // We then create a reference to it and pass it to borrow_reference
    println!("I still own the book {}", title);  
    // Prints "I still own the book Rust Programming"
}
    
fn borrow_reference(s: &String) {  
// borrow_reference takes a reference to a String
    println!("I am borrowing {}", s);  

}  
// Note, we only have a reference to s, so the owner doesn't change

*/

/*
Borrowing Mutable References

Rust also allows you to pass a mutable variable to a function without transfering ownership.
Mutable references marked with &mut are modifiable pointers. Here's  example:

fn main() {
    let mut title = String::from("Rust");  // We have a mutable String variable
    edit(&mut title);  // We pass a mutable reference to edit
    println!("Edited title: {}", title);  // Once edit returns, prints "Edited title: Rust Programming"
}
    
fn edit(title: &mut String) {  // edit takes a mutable reference to a String
    title.push_str(" Programming");  // It can therefore modify the String
}  // After the function call, the borrowed reference is dropped.

*/

/*
Rules of Mutable References
Rust imposes two safe rules on mutable references:

1) No limit on the number of immutable references.
Only one mutable reference or multiple immutable references concurrently, not both.
These rules are necessary to prevent data races. 
2) A data race occurs when two parts of a program try to access a piece of data at the same time. 
For example, if there is a mutable and immutable reference to the same piece of memory, 
the immutable reference might read the data before or after it has been changed by the mutable reference.

Here's an example:

fn main() {
    let mut title = String::from("Rust");  // A mutable variable
    let ref1 = &title;  // An immutable reference
    let ref2 = &title;  // Another immutable reference
    let ref3 = &mut title;  // A mutable reference - will raise a compilation error

    println!("{} {} {}", ref1, ref2, ref3);
}
*/


/*
Dangling References
fn main() {
    let title = bad_reference();  
// This will cause a compile error
}
    
fn bad_reference() -> &String {  
// bad_reference attempts to return a reference to a local variable
    let title = String::from("Rust Programming");  
// A local variable String is created
    &title  
// We then return a reference pointing to it
}  
// The function ends here, and title gets dropped, creating an unreachable (a.k.a. dangling) reference
*/

//Practice

/*
fn main() {
    let book_title = String::from("Rust Programming Basics");

    borrow_book(&book_title);  
    // Borrow the title without transferring ownership
    println!("The library still owns: {}", book_title);  
    // Ownership of book_title remains with the library

    let mut mutable_title = String::from("Learn ");
    edit_title(&mut mutable_title);  
    // Mutably borrow to modify the title
    println!("Updated title: {}", mutable_title); 
    // The modified title
}

fn borrow_book(title: &String) {
    println!("Currently borrowed: {}", title);  
    // Just borrowing, nothing modified
}

fn edit_title(title: &mut String) {
    title.push_str("Rust Programming");  
    // Modify the title by appending to it
}
*/

/*
fn main() {
    let mut books = String::from("The Rust Book, Rust Programming, Programming in Rust");
    manage_books(&mut books);
    println!("Updated books: {}", books);
}

fn manage_books(book_list: &mut String) {
    book_list.push_str(", The Art of Rust");
}
*/

// TODO: Write a function that takes a reference to a String and prints it
fn r(title: &String){
    println!("{}",title);
}

fn main() {
    // TODO: Create a new String variable named book_title
    let book_title = String::from("Rust Programming");
    // TODO: Borrow the book_title using the function
    r(&book_title);
    // TODO: Print the book_title to show that ownership is retained
    println!("{}", book_title);
}