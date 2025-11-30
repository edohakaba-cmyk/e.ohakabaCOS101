use std::fs::File;
use std::io::{Write, Result};

struct Student {
    name: String,
    matric: String,
    department: String,
    level: u32,
}

fn main() -> Result<()> {
    // ====== STUDENT DATA STORED IN A VECTOR ======
    let students = vec![
        Student {
            name: "Oluchi Mordi".to_string(),
            matric: "ACC10211111".to_string(),
            department: "Accounting".to_string(),
            level: 100,
        },
        Student {
            name: "Adams Aliyu".to_string(),
            matric: "ECO10110101".to_string(),
            department: "Economics".to_string(),
            level: 300,
        },
        Student {
            name: "Shania Bolade".to_string(),
            matric: "CSC10328828".to_string(),
            department: "Computer".to_string(),
            level: 200,
        },
        Student {
            name: "Adekunle Gold".to_string(),
            matric: "EEE10202002".to_string(),
            department: "Electrical".to_string(),
            level: 100,
        },
        Student {
            name: "Blanca Edemoh".to_string(),
            matric: "MEE10202001".to_string(),
            department: "Mechanical".to_string(),
            level: 100,
        },
    ];

    // ====== DISPLAY DATA IN TABLE FORMAT ======
    println!("PAU SMIS");
    println!(
        "{:<20} {:<15} {:<15} {:<5}",
        "Student Name", "Matric Number", "Department", "Level"
    );
    println!("{}", "-".repeat(60));

    for s in &students {
        println!(
            "{:<20} {:<15} {:<15} {:<5}",
            s.name, s.matric, s.department, s.level
        );
    }

    // ====== SAVE INTO A FILE ======
    let mut file = File::create("students.csv")?;

    writeln!(file, "PAU SMIS")?;
    writeln!(file, "Student Name,Matric Number,Department,Level")?;

    for s in &students {
        writeln!(
            file,
            "{},{},{},{}",
            s.name, s.matric, s.department, s.level
        )?;
    }

    println!("\nData successfully saved to students.csv");

    Ok(())
}
