use std::io;
use std::f64; // for handling floating point math

fn main() {
    // Input variables
    let mut a_input = String::new();
    let mut b_input = String::new();
    let mut c_input = String::new();

    println!("Enter the value of a: ");
    io::stdin().read_line(&mut a_input).expect("Failed to read input");

    println!("Enter the value of b: ");
    io::stdin().read_line(&mut b_input).expect("Failed to read input");

    println!("Enter the value of c: ");
    io::stdin().read_line(&mut c_input).expect("Failed to read input");

    // Convert input to f64 (floating point)
    let a: f64 = a_input.trim().parse().expect("Please enter a valid number for a");
    let b: f64 = b_input.trim().parse().expect("Please enter a valid number for b");
    let c: f64 = c_input.trim().parse().expect("Please enter a valid number for c");

    // Calculate discriminant
    let discriminant = b * b - 4.0 * a * c;

    println!("\nThe discriminant (b² - 4ac) = {}", discriminant);

    // Determine the nature of roots
    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("There are two distinct real roots:");
        println!("Root 1 = {}", root1);
        println!("Root 2 = {}", root2);
    } else if discriminant == 0.0 {
        let root = -b / (2.0 * a);
        println!("There is one real root:");
        println!("Root = {}", root);
    } else {
        println!("There are no real roots (the roots are imaginary).");
    }
}


