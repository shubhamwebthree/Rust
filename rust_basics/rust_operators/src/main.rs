/*
Rust features six comparison operators: 
equal to (==), 
not equal to (!=), 
greater than (>), 
less than (<), 
greater than or equal to (>=), 
less than or equal to (<=). 
Each of these produces either true or false, referred to as boolean values.
*/


//code practice

fn main() {
    // Scenario: Determining if the submarine is within a safe operating depth
    let max_safe_depth = 300; // maximum safe operating depth in meters
    let submarine_depth = 350; // current submarine depth in meters

    // TODO: Create a new boolean variable "is_too_deep_for_operation" and update the print statement accordingly
    let is_too_deep_for_operation:bool = submarine_depth > max_safe_depth;
    println!("Is the submarine within a safe operating depth? {}", is_too_deep_for_operation);
}


fn main() {
    let current_speed = 8; // Speed of ocean current in knots
    let submarine_speed = 15; // Speed of the submarine in knots

    // TODO: Determine if the submarine can travel faster than the ocean current and print the result
    let can_travel_fast :bool = current_speed < submarine_speed;
    println!("Is our submarine faster than the ocean current? {}",can_travel_fast);
}

/*
fn main() {
    
// TODO: Assign the distance to the harbor (40,000 meters) to a variable
let distance_to_harbor = 40000;
    
    // TODO: Assign the distance to the archipelago (120,000 meters) to another variable
    let distance_to_archipelago = 120000;
    
    // TODO: Compare the distance to the harbor to see if it's less than 500,000 meters
    let compare_distance : bool = distance_to_harbor < 500000;

    // TODO: Check if the distance to the archipelago is greater than the distance to the harbor
    let check_distance_to_archipelago : bool = distance_to_archipelago > distance_to_harbor;
    
    // TODO: Print results of these two comparisons to the console
    
    println!("distance comparison of harbor {} " , compare_distance);
    println!("distance of the archipelago distance is greater than harbor {}", check_distance_to_archipelago);
    
}
*/