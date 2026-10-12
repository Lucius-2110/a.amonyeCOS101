use std::io;

fn read_number(message: &str) -> f64 {
    let mut input = String::new();
    println!("{}", message);
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let num: f64 = input.trim().parse().expect("Please enter a number");
    return num;
}

fn trapezium() -> f64 {
    let height = read_number("Enter the height:");
    let base1 = read_number("Enter base 1:");
    let base2 = read_number("Enter base 2:");
    return height / 2.0 * (base1 + base2);
}

fn rhombus() -> f64 {
    let d1 = read_number("Enter diagonal 1:");
    let d2 = read_number("Enter diagonal 2:");
    return 0.5 * d1 * d2;
}

fn parallelogram() -> f64 {
    let base = read_number("Enter the base:");
    let altitude = read_number("Enter the altitude:");
    return base * altitude;
}

fn cube() -> f64 {
    let side = read_number("Enter the length of side:");
    return 6.0 * side * side;
}

fn cylinder() -> f64 {
    let pi: f64 = 22.0 / 7.0;
    let radius = read_number("Enter the radius:");
    let height = read_number("Enter the height:");
    return pi * radius * radius * height;
}

fn main() {
    println!("Shape Calculator");
    println!("1. Trapezium (area)");
    println!("2. Rhombus (area)");
    println!("3. Parallelogram (area)");
    println!("4. Cube (surface area)");
    println!("5. Cylinder (volume)");

    let mut input = String::new();
    println!("Choose a shape from 1-5:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let choice: i32 = input.trim().parse().expect("Please enter a number");

    if choice == 1 {
        println!("Area of trapezium = {}", trapezium());
    } else if choice == 2 {
        println!("Area of rhombus = {}", rhombus());
    } else if choice == 3 {
        println!("Area of parallelogram = {}", parallelogram());
    } else if choice == 4 {
        println!("Surface area of cube = {}", cube());
    } else if choice == 5 {
        println!("Volume of cylinder = {}", cylinder());
    } else {
        println!("Invalid choice, pick a number from 1 to 5");
    }
}