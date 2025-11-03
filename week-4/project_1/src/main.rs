use std::io;

fn main() {
    
    let mut experience = String::new();
    println!("Is the employee experienced? (yes/no): ");
    io::stdin().read_line(&mut experience).expect("Failed to read input");
    let experience = experience.trim().to_lowercase();

    // Input: age
    let mut age_input = String::new();
    println!("Enter the employee's age: ");
    io::stdin().read_line(&mut age_input).expect("Failed to read input");
    let age: u32 = age_input.trim().parse().expect("Please enter a valid number");

    let incentive: u32;

    if experience == "yes" {
        if age >= 40 {
            incentive = 1_560_000;
        } else if age >= 30 && age < 40 {
            incentive = 1_480_000;
        } else if age < 28 {
            incentive = 1_300_000;
        } else {
            // For ages between 28–29, not explicitly stated; we can assume same as 1,300,000
            incentive = 1_300_000;
        }
    } else {
        incentive = 100_000;
    }

    println!("The employee's annual incentive is: ₦{}", incentive);
}


