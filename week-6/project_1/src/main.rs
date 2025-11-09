use std::io;

fn main() {
    println!("===============================");
    println!("         FOOD MENU");
    println!("===============================");
    println!("P = Poundo Yam & Edinkaiko Soup   ₦3,200");
    println!("F = Fried Rice & Chicken          ₦3,000");
    println!("A = Amala & Ewedu Soup            ₦2,500");
    println!("E = Eba & Egusi Soup              ₦2,000");
    println!("W = White Rice & Stew             ₦2,500");
    println!("===============================");

    // Ask customer for food choice
    let mut food_choice = String::new();
    println!("Enter the food type (P/F/A/E/W): ");
    io::stdin().read_line(&mut food_choice).expect("Failed to read input");
    let food_type = food_choice.trim().to_uppercase();

    // Ask quantity
    let mut quantity_input = String::new();
    println!("Enter quantity: ");
    io::stdin().read_line(&mut quantity_input).expect("Failed to read input");
    let quantity: i32 = quantity_input.trim().parse().expect("Please enter a valid number");

 
    let price = match food_type.as_str() {
        "P" => 3200,
        "F" => 3000,
        "A" => 2500,
        "E" => 2000,
        "W" => 2500,
        _ => {
            println!("Not on the menu");
            return;
        }
    };

    // Calculate total
    let mut total = price * quantity;

    // Apply 5% discount if total > 10,000
    if total > 10_000 {
        let discount = total as f32 * 0.05;
        total = (total as f32 - discount) as i32;
        println!("\nYou got a 5% discount.");
    }

    // Display total
    println!("\n===============================");
    println!("ORDER SUMMARY");
    println!("===============================");
    println!("Food Type: {}", food_type);
    println!("Quantity: {}", quantity);
    println!("Total: {}", total);
    println!("===============================");
    println!("Thank you ");
}

