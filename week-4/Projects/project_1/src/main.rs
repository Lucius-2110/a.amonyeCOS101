use std::io;

fn main() {
    
    // value for a
    println!("Value of a:");
    let mut input_a = String::new();
    io::stdin().read_line(&mut input_a).expect("not a valid number");
    let a: f32 = input_a.trim().parse().expect("not a valid number");

    // value for b
    println!("value of b:");
    let mut input_b = String::new();
    io::stdin().read_line(&mut input_b).expect("not a valid number");
    let b: f32 = input_b.trim().parse().expect("not a valid number");

    // input for c
    println!("value of c:");
    let mut input_c = String::new();
    io::stdin().read_line(&mut input_c).expect("not a valid number");
    let c: f32 = input_c.trim().parse().expect("not a valid number");

    //value for discriminant
    let d: f32 = (b * b) - (4.0 * a * c);

    //conditions
    if d > 0.0 {
        let root1 = (-b + d.sqrt()) / (2.0 * a);
        let root2 = (-b - d.sqrt()) / (2.0 * a);
        println!("Two distinct roots found:");
        println!("Root 1 = {}", root1);
        println!("Root 2 = {}", root2);
    } else if d == 0.0 {
        let root = -b / (2.0 * a);
        println!("Exactly one real root found:");
        println!("Root = {}", root);
    } else {
        println!("No real roots exist (discriminant is negative).");
    }
}
