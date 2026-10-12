use std::io;

fn main() {
    // one vector for the levels, then one vector per profession
    
    let levels = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];

    let office_admin = vec!["Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO"];
    let academic = vec!["-", "Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"];
    let lawyer = vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];
    let teacher = vec!["Placement", "Classroom Teacher", "Senior Teacher", "Leading Teacher", "Deputy Principal", "Principal"];

    println!("APS Level Checker");
    println!("1. Office Admin");
    println!("2. Academic");
    println!("3. Lawyer");
    println!("4. Teacher");

    let mut input1 = String::new();
    println!("Choose a public servant type from 1-4:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let choice: i32 = input1.trim().parse().expect("Please enter a number");

    // pick the vector that matches the choice
    let positions = if choice == 1 {
        office_admin
    } else if choice == 2 {
        academic
    } else if choice == 3 {
        lawyer
    } else if choice == 4 {
        teacher
    } else {
        println!("Invalid choice, pick a number from 1 to 4");
        return;
    };

    let mut input2 = String::new();
    println!("Enter the position title:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let title = input2.trim().to_lowercase();

    // look through the positions for the title
    let mut found = false;
    for i in 0..positions.len() {
        if positions[i].to_lowercase() == title {
            println!("{} is level {}", positions[i], levels[i]);
            found = true;
        }
    }

    if found == false {
        println!("That position was not found in the table");
    }
}