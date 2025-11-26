use std::io::Write;

fn main(){

    let announce = "Week 9 - Rust file Input & output\n";
    let dept = "Department of computer science";

    let mut file = std::fs::File::create("data.txt").expect("create failed");
    file.write_all("Welcome to rust programming\n".as_bytes()).expect("write failed");
    file.write(dept.as_bytes()).expect("write failed");
    println!("\nData written to file.");

}