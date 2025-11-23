fn main() {
    let names = vec!["Abdul", "Mary", "Kelvin", "Chioma"];
    let years = vec![2, 10, 4, 7];

    let mut highest = years[0];
    let mut person = names[0];

    for i in 0..years.len() {
        if years[i] > highest {
            highest = years[i];
            person = names[i];
        }
    }

    println!("Most Experienced Developer: {}", person);
    println!("Years of Experience: {}", highest);
}