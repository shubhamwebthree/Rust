/*
A function in Rust behaves like a recipe: 
it takes in certain ingredients, or "arguments",
processes them according to a set of instructions, 
and then produces an output, or a "return value"
*/

//Functions with Arguments

fn main() {
    // Call the function with "Alice" and 30
    introduce("Shubham", 27); // Prints: "My name is Alice and I am 30 years old."
}

fn introduce(name: &str, age: i32) {
        println!("My name is {} and I am {} years old.", name, age);
}

//Functions that Give Back: Return Values

fn main() {
    // Call the function
    let total = add_numbers(7, 3);
    println!("The sum is: {}", total); // Prints: The sum is: 10
    let difference = subtract_numbers(7,3);
    println!("The difference is: {}", difference); // Prints: The difference is: 4
}


/*
In the add_numbers function, 
we have num1 and num2 as parameters, and the function returns an integer (i32). 
The sum of num1 and num2 is calculated,
because result is the last expression of the function (without a semicolon), 
it's automatically returned.
*/
fn add_numbers(num1: i32, num2: i32) -> i32 {
        let result = num1 + num2;
        result // Returning result without 'return' keyword. 
}


/*
In the subtract_numbers function, 
we have num1 and num2 as parameters, 
function returns an integer (i32). 
The difference of num1 and num2 is calculated, 
and returned by using the return keyword. 
The print statement does not run 
because the function stops execution after it returns a value.
*/

fn subtract_numbers(num1: i32, num2: i32) -> i32 {
    let result = num1 - num2;
    return result; // Returning result with 'return' keyword
    println!("The function returns early, so this line will not print.");
}







fn display_brewing_result(num_of_potions: i32) {
    println!("We concocted {} potent health potions!", num_of_potions);
}

fn brew_health_potions(vial_count: i32, health_per_vial: i32) -> i32 {
    vial_count * health_per_vial // No semicolon because this value should be returned
}

fn main() {
    display_brewing_result(7);
    let total_health = brew_health_potions(7, 20);
    println!("We brewed health potions with a total of {} health points!", total_health);
}


//examples 

// 01

/*

fn add_batches(batch1: i32, batch2: i32) -> i32 {
    // TODO: Return the average number of cakes
    let average = (batch1 + batch2) / 2;
    println!("average number of cakes are {}", average ); 
    average
}

fn main() {
    let total_cakes = add_batches(10, 20); // Represents the total number of cakes from two batches
    println!("The total number of cakes is: {}", total_cakes);
}

*/


// 02

/*

fn bake_cookies(batch_count: i32) {
    println!("Baking {} batches cookies!", batch_count);
}

fn sell_cookies(batch_count: i32, cookies_per_batch: i32) -> i32 {
    let a = batch_count * cookies_per_batch;
    println!("{}", a ); // Error on this line
    a
}

fn main() {
    bake_cookies(3); // Assuming each batch has the same number of cookies
    let total_cookies = sell_cookies(3, 10); // Selling 3 batches, 10 cookies each
    println!("Total cookies sold: {}", total_cookies);
}

*/

fn calculate_total_price(item_count: i32, price_per_item: i32) -> i32 {
    // TODO: Implement the calculation for the total price using item_count and price_per_item
    item_count * price_per_item
}

fn calculate_total_price(item_count: i32, price_per_item: i32) -> i32 {
    // TODO: Implement the calculation for the total price using item_count and price_per_item
    return item_count * price_per_item
}

fn main() {
    let pies_count = 3;
    let price_per_pie = 15;
    // TODO: Call your function here to calculate the total price for pies, given the quantity and price per pie.
    let total_price = calculate_total_price(pies_count,price_per_pie);
    
    println!("Total price for pies: ${}", total_price);
}


// TODO: Define a function to calculate the area of a field.
// This function should accept 2 arguments for length and width and return the calculated area as a float.
fn calculate_area (length:f32, width:f32) -> f32  {
    return length * width
}

fn main() {
    // TODO: Call your function with values flet arearea = or length and width and print the result
    let area = calculate_area(12.0, 21.0);
    println!("calculated area is {}", area);
}