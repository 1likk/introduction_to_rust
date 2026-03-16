use std::io;

pub fn convert() {
    println!("Write temperature in Farenheit: ");

    let mut fahrenheit = String::new();

    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Failed to read line");
    
    let fahrenheit: f64 = fahrenheit
        .trim()
        .parse()
        .expect("Please write digit!");
    
    let celsius = (fahrenheit - 32.0)/1.8;
    println!("Result: {celsius}");
}