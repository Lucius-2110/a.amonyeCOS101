fn main() {
    let candidates: Vec<(&str, u32)> = vec![
        ("Chudi", 4),
        ("Zay", 7),
        ("Lebron", 3),
        ("Messi", 9),
        ("Kilo-mene", 6),
    ];

    // starts w the first person as the highest so far
    let mut top_name = candidates[0].0;
    let mut top_years = candidates[0].1;

    // cycles through every candidate in the interview
    for i in 0..candidates.len() {
        println!("{} has {} years of experience", candidates[i].0, candidates[i].1);

        if candidates[i].1 > top_years {
            top_name = candidates[i].0;
            top_years = candidates[i].1;
        }
    }

    println!();
    println!("Highest experience: {} with {} years", top_name, top_years);
}