/*
1.In Rust, variables are divided into two categories: mutable and immutable. 
2.By default, all variables in Rust are immutable, 
which means once a value is assigned to them, it cannot be changed.
3.If you attempt to alter the value of an immutable variable, 
the Rust compiler will throw an error.    let mut number_of_moon = 181;
    println!("Initial number of moons: {}", number_of_moon);
    // TODO: New moons have been discovered! Change the number of moons to 184
    number_of_moon = 184;
    // TODO: Print out the speed of light and the updated number of moons using println!
    println!("The speed of ligh is {} and updated number of moon is {}",SPEED_OF_LIGHT, number_of_moon);
}
4.However, Rust gives you the ability to explicitly make a variable mutable using the mut keyword. 
5.A mutable variable is one whose value can be changed after it has been initially declared and assigned.
*/

fn main() {
    // TODO: Declare a constant for the speed of light in a vacuum with a value of 299792 (km/s)
const SPEED_OF_LIGHT :i32 = 299792;
    // TODO: Declare a mutable variable for the number of moons in the Solar System, starting at 181
