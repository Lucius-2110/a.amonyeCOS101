use std::fs::File;
use std::io::Write;

fn main() {
    let lager = ["33 Export", "Desperados", "Goldberg", "Gulder", "Fayrouz", "Heineken", "Star"];
    let stout = ["Legend", "Turbo King", "Williams"];
    let non_alcoholic = ["Maltina", "Amstel Malta", "Malta Gold"];

    // the txt file
    let mut file = File::create("drinks.txt").expect("create failed");

    file.write_all("Nigerian brewery limited - categories of drinks\n".as_bytes()).expect("write failed");

    file.write_all("\nLager\n".as_bytes()).expect("write failed");
    for drink in lager.iter() {
        file.write_all(drink.as_bytes()).expect("write failed");
        file.write_all("\n".as_bytes()).expect("write failed");
    }

    file.write_all("\nStout\n".as_bytes()).expect("write failed");
    for drink in stout.iter() {
        file.write_all(drink.as_bytes()).expect("write failed");
        file.write_all("\n".as_bytes()).expect("write failed");
    }

    file.write_all("\nNon-Alcoholic\n".as_bytes()).expect("write failed");
    for drink in non_alcoholic.iter() {
        file.write_all(drink.as_bytes()).expect("write failed");
        file.write_all("\n".as_bytes()).expect("write failed");
    }

    println!("drinks.txt created");
}
