fn main() {
    let n1 = "Electrical".to_string();
    let n2 = " Electronic".to_string();
    let n3 = " Engineering".to_string();
    let n4 = n1 + &n2 + &n3; 

    println!("\nThe {} is informed by...", n4);

    let w1 = "Computer".to_string();
    let w2 = " Science".to_string();
    let w3 = w1 + &w2; 
    println!("\n{} is aimed at...", w3);
}