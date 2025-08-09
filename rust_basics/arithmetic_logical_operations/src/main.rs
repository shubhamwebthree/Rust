/*
Rust's primitive data types include 
i32 for whole numbers, 
f32 for decimal numbers, 
bool for true/false values, 
and char for single characters.
--------------------------------
addition (+), 
subtraction (-), 
multiplication (*), 
division (/), 
and modulus (which represents the remainder of the division, %) 
*/


/*

fn main() {
    // Define the speed of a car in km/h
    let speed_in_kmh = 120;
    
    // Time traveled
    let time_in_hours = 3;
    
    // Calculate the distance covered
    let distance_covered = speed_in_kmh * time_in_hours;
    
    // Output the result
    println!("Car traveling at {} km/h for {} hours covering a distance of {} kilometers.", speed_in_kmh, time_in_hours, distance_covered);
    
    // Check if the speed limit is exceeded. Assume the speed limit to be 90 km/h
    let speed_limit = 90;
    
    // Output the result
    println!("Is the speed limit exceeded? {}", speed_in_kmh > speed_limit);
}
 
 */

/*
fn main() {
    // Variables representing the level of oxygen in the submarine and the required minimum before dive.
    let current_oxygen: i32 = 150;
    let required_oxygen: i32 = 150;
    let hull_integrity_check: bool = false;

    // Check if there is adequate oxygen AND if the pre-dive hull integrity check is complete.
    println!("{}", current_oxygen >= required_oxygen || hull_integrity_check);
}
 */


 /*
 
 fn main() {
    let books_counted: i32 = 17;
    let magazines_counted: i32 = 3;
    let research_papers_found: bool = false;

    // TODO: Calculate the total number of items in our library.
    let total_items = books_counted + magazines_counted;
    println!("Total items in the library: {}", total_items);

    // TODO: Use a logical AND operation to check if:
    println!("Found more than 15 books and research papers: {}", books_counted > 15 && research_papers_found  );
}

*/

fn main() {

    // TODO: Declare and initialize variables for battery capacity (15,000 mAh), power consumption (120 mAh/hour), and patrol duration (2 hours)
    let battery_capacity :i32 = 15000;
    let power_consumption: i32 = 120;
    let patrol_duration :i32 = 2;

    // TODO: Calculate the remaining power after the patrol
    let remaining_power: i32 = battery_capacity - (power_consumption * patrol_duration);
    
    // TODO: Use a logical operation to determine if the remaining power is more than half of the battery capacity
    let more_than_half_of_battery  = remaining_power > (battery_capacity/2) ; 

    // TODO: Print out the result in a format: "Enough power for another round? true/false"
    
    println!("Enough power for another round ? {}", more_than_half_of_battery);
}