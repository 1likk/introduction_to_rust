use std::io;

pub fn fibon() {
    println!("Please write number of fibonacci: "); 
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    let n: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please, write a integer number");
            return;
        }
    };

    let mut a: u128 = 0;
    let mut b: u128 = 1;

    if n == 0 {
        println!("Number fibo at №0 is: {a}");

    } else if n == 1 {
        println!("Number fibo at №1 is: {b}");
    } else {
        for _ in 2..=n {
            let next = a + b;
            a = b;
            b = next; 
        }
        println!{"Number fibo at №{n} is: {b}"}

    }
}