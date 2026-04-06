fn main() {
    //panic!("crash and burn");
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
}

fn example() {
    let v = vec![1, 2, 3];
    
    v[99];
}

