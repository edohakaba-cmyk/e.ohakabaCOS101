fn main() {
    // tuple with datatypes listed
    let datatype_tuple: (&str, f32, u8) = ("Rust", 3.77, 97);
    println!("Tuple contents = {:?}",datatype_tuple);
    
    //tuple without datatypes listed
    let no_datatype_tuple = ("Rust", "fun", 199);
    println!("Tuple contenets = {:?}", no_datatype_tuple );

    //accessing tuple element at  index 0
    println!("Value at index 0 = {}",datatype_tuple.0 );

    //accessing tuple element at  index 1
    println!("Value at index 1 = {}",datatype_tuple.1 );

    //accessing tuple element at  index 2
    println!("Value at index 2 = {}",datatype_tuple.2 );
}
