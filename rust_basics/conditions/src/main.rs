/*
fn main() {
    let rainfall = 4; // Rainfall amount on a scale from 0 to 5

    // Using a match expression to handle actions depending on rainfall level
    match rainfall {
        0 => println!("No umbrella needed, clear skies!"),
        1 => println!("Slight chance of drizzle, carry a pocket umbrella maybe."),
        2 => println!("Moderate rain, don't forget your umbrella!"),
        3 => println!("Heavy rain, make sure to wear waterproof gear!"),
        4 => println!("Very heavy rain, stay indoors if possible!"),
        _ => println!("Invalid rainfall level."),
    }
}
*/

fn main() {
    let temperature_celsius = 18; // Expected temperature in degrees celsius
    
    if temperature_celsius < 15 { // TODO: check if the temperature is less than 15
        // Print a message for cold weather coding
        println!("{}is  cold weather coding for", temperature_celsius);
    } else if temperature_celsius < 25 { // TODO: check if the temperature is less than 25
        // Print a message for moderate weather coding
        println!("{} moderate weather coding", temperature_celsius);
    } else {
        // Print a message for hot weather coding
        println!("{} hot weather fro coding", temperature_celsius);
    }
}