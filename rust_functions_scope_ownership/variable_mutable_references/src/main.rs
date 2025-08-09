/*
A variable reference is like having a house address—
it guides you to where the house is located. 
Just as you can visit and look at the house based on its address, 
but not alter it, a variable reference allows you to see a value without changing it.

To create a reference to a variable, 
add & before the variable name. 
To dereference a variable and get the value of a reference variable, 
add * before the variable name.

*/

// Variable References

/*
fn main() {
    let x = 5;
    let y = &x; // y is a reference to the memory location of x
    let z = *y; // dereference y
    println!("The value at y is {}", z); // Prints: "The value at y is 5"
}
*/


// Mutable References

/*

fn add_one(x: &mut i32) { // function taking a mutable reference to an i32
    *x += 1; // increments the value the reference points to
}

fn main() {
    let mut num = 5; // our mutable variable
    add_one(&mut num); // pass a mutable reference to our variable
    println!("The value of num is: {}", num); // prints: The value of num is: 6
}

fn add_one(x: &mut i32) { — Here, we declare add_one as a function that takes a mutable reference to a 32-bit integer.
*x += 1; — We increment the value to which the mutable reference x points.
let mut num = 5; — We declare a mutable integer num and assign it the value 5.
add_one(&mut num); — We call the add_one function and pass it the mutable reference of num.
println!("The value of num is: {}", num); — We print the new value of num, which is now 6, thanks to our add_one function.

*/

// Mutable References and Scope

/*
Here's how the concept of scope connects with mutable references:
In the main function, we have the num variable — let's think of it as our own house. We're comfortable letting add_one temporarily hold the key to our house (the mutable reference to our num variable) because we trust that add_one will only make the specific change we have agreed on.
This agreement is referred to as 'scope'. The add_one function has a scope that is clearly defined by its function body. It can only make changes within this scope.
Once add_one does its job, that is, increments our num variable by one, its scope ends. It steps out, hands the key back, and it can no longer make any changes to our house. The key no longer works beyond the end of the add_one function, and the mutable reference it once had is no longer valid.
This mechanism is crucial for preserving data integrity as it ensures that changes made by functions are controlled, traceable, and limited to the intended scope. We rest assured that our house – or variable value – only changes when and where we expect.
*/

// examples

/*
fn buy_apples(apple_count: &mut i32, apples_bought: i32) {
    *apple_count += apples_bought;
}

fn main() {
    let mut my_apple_count = 5; // Starting with 5 apples
    let shopping_cart = &mut my_apple_count;
    buy_apples(shopping_cart, 10); // Buying 10 more apples
    println!("I had {} apples; now I have {}!", 5, my_apple_count);
}
*/

/*
fn main() {
    let mut house_paint_color = String::from("Red");
    let house_address = &mut house_paint_color;
    *house_address = String::from("Blue");
    // TODO: Update the `house_paint_color` to "Blue" using `house_address`.
    
    println!("The house is now painted in {}", house_paint_color);
}
*/

fn paint_walls(color_address: &mut String) {
    // TODO: Change the color value to something vibrant!
    *color_address = String::from("Yellow");
}

fn main() {
    // TODO: Declare a mutable variable `wall_color` with the initial value "White".
    let mut wall_color = String::from("White");   
    // TODO: Call `paint_walls` to change `wall_color`.
    paint_walls(&mut wall_color);
    // TODO: Print out the new color of the walls.
    println!("New color of wall is {}", wall_color);
}