// Ownership in Rust

/*
fn main() {
    let book_title = String::from("Rust Programming for Beginners");
    let book_title = create_book_title(book_title); // Ownership transferred here
    println!("The book's title is: {}", book_title); // Ownership is used here
}

fn create_book_title(title: String) -> String { 
    title // Ownership is returned to the main function
}
*/

/*
fn main() {
    let book_title = String::from("The Little Prince"); 
    // TODO: Transfer the ownership of book_title to a function that logs the book's title
    library_log_book(book_title);
}

fn library_log_book(title: String) {
    // TODO: Implement the body to print the title of the book as "A new book added: [book_title]"
    println!("A new book {} added", title);
}
*/

/*
fn main() {
    let book1 = String::from("Rust Programming for Beginners");
    let book2 = check_title(book1); // Transfer ownership from book1 to book2
    
    println!("Book Title: {}", book2); // Hint: What variable owns the actual value?
}

fn check_title(book_title: String) -> String {
    book_title
}
*/

/*
fn main() {
    let is_available:bool = true; // Hint: Should this variable be a copy or non-copy data type?
    let in_stock = is_available; 
    println!("Book status: Available - {}, In stock: - {}", is_available, in_stock);
}
*/

// TODO: Define a function `lend_book` that takes ownership of a book and prints a message "{Book title} is being read by a friend". It should then return ownership.
fn lend_book(book_title:String)-> String {
    println!("{} is being read by a friend",book_title);
    book_title
}

fn main() {
    // TODO: Create a `String` variable with a book title.
    let book_title = String::from("niRa");
    // TODO: Pass ownership of the book to `lend_book` and retrieve it back.
    let book_title = lend_book(book_title);
    // TODO: Print a message "My friend has finished reading {Book title}"
    println!("My friend has finished reading {}",book_title);
}