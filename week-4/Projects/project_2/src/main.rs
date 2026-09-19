//code to calculate employee annual incentive
use std::io;

fn main() {

    //experience 
    println!("Is the employee experienced? ('yes' or 'no'):");
    let mut exp_input = String::new();
    io::stdin().read_line(&mut exp_input).expect("this is not a valid answer");
    let exp = exp_input.trim().to_lowercase();

    //age
    println!("Enter employee age:");
    let mut age_input = String::new();
    io::stdin().read_line(&mut age_input).expect("this is not a valid number");
    let age: i64 = age_input.trim().parse().expect("this is not a valid number");

    if exp == "yes" {
        if age >= 40 {
            println!("Annual Incentive: N1,560,000");
        } else if age >= 30 && age <= 39 {
            println!("Annual Incentive: N1,480,000");
        } else if age < 28 {
            println!("Annual Incentive: N1,300,000");
        } else {
            println!("No annual incentive for this age.");
        }
    } else {
        println!("Annual Incentive: N100,000");
    }
}