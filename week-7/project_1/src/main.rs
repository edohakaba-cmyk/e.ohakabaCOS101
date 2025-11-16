use std::io;

// --- FUNCTIONS FOR EACH SHAPE ---

fn area_trapezium(height: f64, base1: f64, base2: f64) -> f64 {
    height / 2.0 * (base1 + base2)
}

fn area_rhombus(diagonal1: f64, diagonal2: f64) -> f64 {
    0.5 * diagonal1 * diagonal2
}

fn area_parallelogram(base: f64, altitude: f64) -> f64 {
    base * altitude
}

fn area_cube(side: f64) -> f64 {
    6.0 * side.powi(2)
}

fn volume_cylinder(radius: f64, height: f64) -> f64 {
    std::f64::consts::PI * radius.powi(2) * height
}

// --- MAIN PROGRAM ---

fn main() {
    println!("MTH 101 - AREA & VOLUME CALCULATOR");
    println!("Select a shape to calculate:");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Volume of Cylinder");
    println!("Enter your choice (1-5):");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice: u32 = choice.trim().parse().unwrap_or(0);

    match choice {
        1 => {
            // Trapezium
            let height = read_value("Enter height:");
            let base1 = read_value("Enter base1:");
            let base2 = read_value("Enter base2:");
            let result = area_trapezium(height, base1, base2);
            println!("Area of Trapezium = {}", result);
        }

        2 => {
            // Rhombus
            let d1 = read_value("Enter diagonal 1:");
            let d2 = read_value("Enter diagonal 2:");
            let result = area_rhombus(d1, d2);
            println!("Area of Rhombus = {}", result);
        }

        3 => {
            // Parallelogram
            let base = read_value("Enter base:");
            let altitude = read_value("Enter altitude:");
            let result = area_parallelogram(base, altitude);
            println!("Area of Parallelogram = {}", result);
        }

        4 => {
            // Cube
            let side = read_value("Enter side length:");
            let result = area_cube(side);
            println!("Area of Cube = {}", result);
        }

        5 => {
            // Cylinder
            let radius = read_value("Enter radius:");
            let height = read_value("Enter height:");
            let result = volume_cylinder(radius, height);
            println!("Volume of Cylinder = {}", result);
        }

        _ => println!("Invalid choice! Please run the program again."),
    }
}

// --- INPUT FUNCTION (to avoid repeating code) ---
fn read_value(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Invalid number")
}
