use std::fs::File;
use std::io::{Write, Result};

fn main() -> Result<()> {
    // ====== DATASET 1: COMMISSIONERS ======
    let commissioners = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etiyie",
    ];

    // ====== DATASET 2: MINISTRIES ======
    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    // ====== DATASET 3: GEOPOLITICAL ZONES ======
    let zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    // ====== DISPLAY MERGED TABLE ======
    println!(
        "{:<5} {:<30} {:<20} {:<15}",
        "S/N", "Name of Commissioner", "Ministry", "Geo-Political Zone"
    );
    println!("{}", "-".repeat(85));

    for i in 0..commissioners.len() {
        println!(
            "{:<5} {:<30} {:<20} {:<15}",
            i + 1,
            commissioners[i],
            ministries[i],
            zones[i]
        );
    }

    // ====== SAVE MERGED DATA INTO A FILE ======
    let mut file = File::create("efcc_convicted_ministers.csv")?;
    writeln!(file, "S/N,Name of Commissioner,Ministry,Geo-Political Zone")?;

    for i in 0..commissioners.len() {
        writeln!(
            file,
            "{},{},{},{}",
            i + 1,
            commissioners[i],
            ministries[i],
            zones[i]
        )?;
    }

    println!("\nFile saved as: efcc_convicted_ministers.csv");

    Ok(())
}
