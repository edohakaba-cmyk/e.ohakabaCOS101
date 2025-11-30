use std::fs::File;
use std::io::{Write, Result};

fn main() -> Result<()> {
    // ====== DRINK CATEGORIES STORED IN VECTORS ======
    let lager = vec![
        "33 Export",
        "Desperados",
        "Goldberg",
        "Gulder",
        "Heineken",
        "Star",
    ];

    let stout = vec![
        "Legend",
        "Turbo King",
        "Williams",
        "",        // to match table rows
        "",
        "",
    ];

    let non_alcoholic = vec![
        "Maltina",
        "Amstel Malta",
        "Malta Gold",
        "Fayrouz",
        "",
        "",
    ];

    // ====== DISPLAY THE TABLE ON SCREEN ======
    println!("{:<15} {:<15} {:<15}", "Lager", "Stout", "Non-Alcoholic");
    println!("{}", "-".repeat(45));

    for i in 0..lager.len() {
        println!(
            "{:<15} {:<15} {:<15}",
            lager[i],
            stout[i],
            non_alcoholic[i]
        );
    }

    // ====== SAVE TABLE INTO A FILE ======
    let mut file = File::create("nigerian_breweries_drinks.csv")?;

    writeln!(file, "Lager,Stout,Non-Alcoholic")?;

    for i in 0..lager.len() {
        writeln!(
            file,
            "{},{},{}",
            lager[i],
            stout[i],
            non_alcoholic[i]
        )?;
    }

    println!("\nFile saved as: nigerian_breweries_drinks.csv");

    Ok(())
}
