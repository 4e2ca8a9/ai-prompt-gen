use character_generator::generate_character;

fn main() {
    let ages = [3, 5, 8, 11, 13, 15, 17, 20, 25, 30];

    for &age in &ages {
        let character = generate_character(age);
        println!("{character}");
    }

    // Show a short description
    let c = generate_character(16);
    println!("Short description:");
    println!("{}", c.short_describe("Mia"));
}
