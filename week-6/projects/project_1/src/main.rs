use std::io;

fn main() {
    // menu part
    println!("            WELCOME TO MAMA PUT          ");
    println!("                                         ");
    println!("P -> Poundo Yam / Edinkaiko Soup : N3,200");
    println!("F -> Fried Rice & Chicken        : N3,000");
    println!("A -> Amala & Ewedu Soup          : N2,500");
    println!("E -> Eba & Egusi Soup            : N2,000");
    println!("W -> White Rice & Stew           : N2,500");
    println!("                                         ");

    // food type selector
    println!("select food type (P, F, A, E, W): ");
    let mut food_type = String::new();
    io::stdin().read_line(&mut food_type).expect("Failed to read input");
    
    // Clean input and extract the upper-case code letter
    let food = food_type.trim().to_uppercase();

    // Determine unit price based on user code
    let unit_price: f64 = if food == "P" {
        3200.0
    } else if food == "F" {
        3000.0
    } else if food == "A" {
        2500.0
    } else if food == "E" {
        2000.0
    } else if food == "W" {
        2500.0
    } else {
        println!("Invalid option selected!");
        return;
    };

    //quantity
    println!("Enter quantity: ");
    let mut qty_input = String::new();
    io::stdin().read_line(&mut qty_input).expect("Failed to read input");
    
    let quantity: f64 = match qty_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        }
    };

    // Subtotal
    let subtotal = unit_price * quantity;
    let mut final_total = subtotal;

    //discount 
    if subtotal > 10000.0 {
        let discount = subtotal * 0.05;
        final_total = subtotal - discount;
        println!("\nDiscount Applied (5%): -N{:.2}", discount);
    } else {
        println!("\nNo discount applied (your order total is less than N10,000 ).");
    }

    // print Total Summary
    println!("Subtotal   : N{:.2}", subtotal);
    println!("Total Due  : N{:.2}", final_total);
}