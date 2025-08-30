//Introduction to While Loops in Rust

/*
Hello! In this lesson, we will dive into while loops in Rust, a fundamental control structure used frequently in programming. 
Understanding while loops is crucial because they allow you to repeat a block of code as long as a specified condition is true. 
This makes them incredibly useful for scenarios where you don't know in advance how many times you need to repeat an operation.


We'll cover the basics of while loops, their syntax, common use cases, handling infinite loops, and scope within while loops.

*/

//Basics of While Loops

/*
A while loop in Rust repeatedly executes a block of code as long as a given condition evaluates to true. 
The general syntax looks like this:

while condition {
    code to execute
}


As long as the condition is true, the block is executed and the condition is checked again. 
Let's take a look at a concrete example:

fn main() {
    let mut count = 0;
    while count < 5 {
        println!("Count is: {}", count);
        count += 1;
    }
}
/* Output:
    Count is: 0
    Count is: 1
    Count is: 2
    Count is: 3
    Count is: 4
*/
*/

//Infinite Loops and Loop Control

/*
While loops can become infinite if their conditions never become false. 
Be cautious to ensure they eventually terminate. For example:

fn main() {
    let mut countdown = 5;
    while countdown > 0 {
        println!("Countdown is: {}", countdown);
    }
}

This loop would never terminate because the countdown value remains unchanged, keeping the condition true. 
To fix this infinite loop, the countdown variable must be decremented as follows:

fn main() {
    let mut countdown = 5;
    while countdown > 0 {
        println!("Countdown is: {}", countdown);
        countdown -= 1;
    }
}
/* Output:
    Countdown is: 5
    Countdown is: 4
    Countdown is: 3
    Countdown is: 2
    Countdown is: 1
*/


*/

//Looping Through Indices

/*
Sometimes, you'll want to iterate over arrays or vectors using indices. While loops can help with this:

fn main() {
    let numbers = [1, 2, 3, 4];
    let length = numbers.len();
    let mut index = 0;
    while index < length {
        println!("Value at index {} is {}", index, numbers[index]);
        index += 1;
    }
}
/* Output:
    Value at index 0 is 1
    Value at index 1 is 2
    Value at index 2 is 3
    Value at index 3 is 4
*/
*/


//Scope within While Loops

/*
In Rust, variable scopes are crucial to understand. 
Variables declared inside a while loop are not accessible outside of it:

fn main() {
    let mut num = 0;
    while num < 5 {
        let doubled = num * 2;
        println!("{} doubled is {}", num, doubled);
        num += 1;
    }
    // println!("Last doubled number is {}", doubled); // Error: doubled is out of scope
}

*/

//Practice

/*
fn main() {
    let mut countdown = 5;
    while countdown > 0 {
        println!("Countdown is: {}", countdown);
        countdown -= 1;
    }
    println!("Lift off!");
}
*/

/*
fn main() {
    let powers = ["Flight", "Super Strength", "Invisibility", "Telepathy"];
    
    // Create a while loop that prints each power in reverse order
    let mut i = powers.len();
    while i > 0 {
        i -= 1;
        println!("{}", powers[i]);
    }
}
*/


/*
fn main() {
    // Define a variable days, set to 1, to keep track of the days
    let mut days = 1;
    
    // Implement a while loop that runs for all 7 days
    while days <= 7 {
        // Calculate the number of heroic deeds for each day (the double of the day number)
        let heroic_deeds = days * 2;
        
        // Print the number of heroic deeds for the current day
        println!("Day {}: {} heroic deeds", days, heroic_deeds);
        
        // Increment the days counter
        days += 1;
    }
}
*/
