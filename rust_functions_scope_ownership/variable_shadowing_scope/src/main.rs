/*
In Rust, variable shadowing is a unique feature that allows you 
to declare a new variable with the same name as a previous variable
In Rust, apart from defining the variable scope, 
you can also use variable shadowing within different scopes. 
Shadowing lets you reuse variable names and apply transformations to values, 
ensuring code clarity while fullfilling different needs in each scope

*/

// examples

/*
fn test(version: i32) {
    println!("Testing software version {}", version);
}

fn main() {
    let version = 1; // The version to release
    
    { // Creating a new scope for the version to test
    let version = 2;
        test(version);
    }

    println!("Releasing software version {}", version);
}

*/

/*
fn main() {
    let mut version = 1;
    {
        version = version + 1; // Incrementing the version number.
        println!("Updated to software version {}", version);
    }
    println!("Current software version {}", version); // Incorrect output due to the bug
    
}    
*/

fn main() {
    let version = 1;
    println!("Current version: {}", version);
    // TODO: Use variable shadowing to update "version" by calling your function and print the updated version.
    let version = update_version(1);
    println!("Updated version is {}", version);
}

fn update_version(version: i32) -> i32 {
    // TODO: Implement the logic to increment the version number by 1 and return the new value
    version + 1
}