mod convert;

fn main(){
    convert::convert();
    let tup = (500, 6.1, 1);
    let (x, y, z) = tup;

    println!("{x}");
    condition();
    loops();
    while_loop();
}

fn condition(){
    let number = 3;

    if number < 5 {
        println!("condition was true");
    }
    else {
        println!("condition was false");
    }
}

fn loops(){
    let mut n = 0;
    loop {
        println!("again again");
        n = n + 1;
        if n == 10 {
            break;
        }
    }
}

fn while_loop(){
    let mut i = 1;
    let mut j = 1;
    while i < 10 {
        while j < 10 {
            print!("{}\t", i * j);
            j = j + 1;
        }
        println!();
        i = i + 1;
        j = 1;
    }
}

