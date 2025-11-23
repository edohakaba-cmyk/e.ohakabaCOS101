use std::io;

fn main() {
    println!("Simple APS Level Checker");
    println!("-------------------------");

    // Very small mapping (from your table)
    println!("Enter a role (examples: Intern, Administrator, Paralegal, Principal):");

    let mut role = String::new();
    io::stdin().read_line(&mut role).unwrap();
    let role = role.trim().to_lowercase();

    let aps_level = if role == "intern" {
        "APS 1–2"
    } else if role == "administrator" {
        "APS 3–5"
    } else if role == "senior administrator" {
        "APS 5–8"
    } else if role == "office manager" {
        "EL1–10"
    } else if role == "director" {
        "EL2 10–13"
    } else if role == "ceo" {
        "SES"
    } else if role == "paralegal" {
        "APS 1–2"
    } else if role == "junior associate" {
        "APS 3–5"
    } else if role == "associate" {
        "APS 5–8"
    } else if role == "senior associate 1-2" {
        "EL1–10"
    } else if role == "senior associate 3-4" {
        "EL2 10–13"
    } else if role == "partner" {
        "SES"
    } else if role == "placement" {
        "APS 1–2"
    } else if role == "classroom teacher" {
        "APS 3–5"
    } else if role == "snr teacher" {
        "APS 5–8"
    } else if role == "leading teacher" {
        "EL1–10"
    } else if role == "deputy principal" {
        "EL2 10–13"
    } else if role == "principal" {
        "SES"
    } else {
        "Unknown role"
    };

    println!("APS Level: {}", aps_level);
}
