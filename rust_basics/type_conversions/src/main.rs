/*
Rust doesn't automatically treat an i32 (integer) as an f64 (floating point), or vice versa.

Here's how to convert an i32 to a f64
let i: i32 = 10;  // an integer 
let d: f64 = i as f64;  // explicit conversion to f64

*/

/*
fn main() {
    let temp_celsius: f32 = 30.0; // Temperature in Celsius
    let temp_fahrenheit: f64 = ( (temp_celsius as f64) * 9.0/5.0) + 32.0; // Convert to Fahrenheit
    
    println!("Temperature in Celsius: {}", temp_celsius);
    println!("Temperature in Fahrenheit: {}", temp_fahrenheit);
}

*/


fn main() {
    // Constant with temperature as a string
    let temperature_string = "23.5";

    // TODO: Convert the temperature_string to a floating-point number and assign it to a variable
    let temperature = temperature_string.parse::<f32>().unwrap();
    
    // Print the temperature in Celsius
    println!("The temperature is {} degrees Celsius.", temperature);
}