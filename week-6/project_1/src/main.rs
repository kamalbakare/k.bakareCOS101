use std::io;

fn main() {
    println!("What would you like to order? Hurry!! We have a 5% discount on orders above N10,000");

    let mut total = 0;

    loop {
        println!("\nMenu:");
        println!("P = Poundo Yam/Edinkaiko Soup - N3200");
        println!("F = Fried Rice & Chicken      - N3000");
        println!("A = Amala & Ewedu Soup        - N2500");
        println!("E = Eba & Egusi Soup          - N2000");
        println!("W = White Rice & Stew         - N2500");

        let mut food_code = String::new();
        println!("Input P or F or A or E or W (type Q to quit): ");
        io::stdin().read_line(&mut food_code).expect("Failed to read input");
        let choice = food_code.trim().to_uppercase();

        if choice == "Q" {
            break;
        }

        let price_per_item = match choice.as_str() {
            "P" => 3200,
            "F" => 3000,
            "A" => 2500,
            "E" => 2000,
            "W" => 2500,
            _ => {
                println!("Invalid code! Please select from the menu.");
                continue;
            }
        };

        let mut quantity_input = String::new();
        println!("Enter quantity:");
        io::stdin().read_line(&mut quantity_input).expect("Failed to read input");

        let quantity: u32 = match quantity_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid quantity! Please enter a valid number.");
                continue;
            }
        };

        total += price_per_item * quantity;
        println!("Item added to your cart. Current total: N{}", total);
    }

    if total > 10000 {
        let discount = (total as f32 * 0.05) as u32;
        total -= discount;
        println!("A discount of 5% applied: N{}", discount);
    }

    println!("\nFinal Total: N{}", total);
}
