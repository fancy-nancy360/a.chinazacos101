use std::io;

fn main() {
    // Create an empty vector "City"
    let mut city: Vec<String> = Vec::new();

    // Print city vector
    println!("The city vector has {} elements", city.len());

    // Push new elements into vector
    let mut input_1 = String::new();
    println!("How many cities do you want to enter? ");
    io::stdin().read_line(&mut input_1).expect("Failed to read input");

    let city_num: u32 = input_1.trim().parse().expect("Invalid input");

    let mut count = 1;
    while count <= city_num {
        let mut input_2 = String::new();
        println!("Enter city {}: ", count);

        io::stdin().read_line(&mut input_2).expect("Failed to read input");

        let new_city = input_2.trim().to_string();
        city.push(new_city);

        count += 1;
    }

    println!("\nYour preferred cities are:\n");

    let mut count = 1;
    // loop to iterate elements in vector
    for i in city {
        println!("{}: {}", count, i);
        count += 1;
    }
}
