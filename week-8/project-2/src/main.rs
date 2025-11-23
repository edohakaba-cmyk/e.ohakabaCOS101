// A Rust program to find the candidate
// with the highest years of programming experience.

struct Candidate {
    name: String,
    years: u32,
}

fn main() {
    // Using a vector (compound data type) of structs (compound data type)
    let candidates = vec![
        Candidate { name: String::from("James"), years: 3 },
        Candidate { name: String::from("Ngozi"), years: 6 },
        Candidate { name: String::from("Aisha"), years: 2 },
        Candidate { name: String::from("Tunde"), years: 8 },
        Candidate { name: String::from("Amaka"), years: 5 },
    ];

    let mut most_experienced = &candidates[0];

    // Loop through the list to find the highest experience
    for c in &candidates {
        if c.years > most_experienced.years {
            most_experienced = c;
        }
    }

    println!("Most experienced candidate:");
    println!("Name: {}", most_experienced.name);
    println!("Years of experience: {}", most_experienced.years);
}
