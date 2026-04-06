fn main() -> Result<(), String> {
    show_result()?;
    let result = divide(10.0, 2.0);
    match result {
        Ok(value) => println!("Result of a divided by b is {}", value),
        Err(err) => panic!("Oopss, problem {err:?}"),
    }
    let value = result.unwrap();
    let value_e = result.expect("Failed to divide");
    
    Ok(())
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        return Err(String::from("Division by zero"));
    }
    
    Ok(a / b)
}

fn show_result() -> Result<(), String> {
    let val = divide(10.0, 0.0)?;
    println!("Result: {}", val);

    Ok(())
}




/*
 panic!("crash and burn");
    let greeting_life_result = File::open("hello.txt");

    let greeting_life = match greeting_life_result {
        Ok(file) => file, 
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };

    example();

fn example() {
let v = vec![1, 2, 3];

v[99];
}  
*/
