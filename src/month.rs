pub fn month () { 
    let gifts = [
        "a Partridge in a Pear Tree",
        "two Turtle Doves",
        "three French Hens",
        "four Calling Birds",
        "five Gold Rings",
        "six Geese a-Laying",
        "seven Swans a-Swimming",
        "eight Maids a-Milking",
        "nine Ladies Dancing",
        "ten Lords a-Leaping",
        "eleven Pipers Piping",
        "twelve Drummers Drumming",
    ];

    for day in 1..=12 {
        let ordinal = match day {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eighth",
        9 => "ninth",
        10 => "tenth",
        11 => "eleventh",
        12 => "twelfth",
        _ => "",
    };
    
    println!("\nOn the {ordinal} day of Christmas my true love gave to me:");

    for gift_index in (0..day).rev() {
        if day > 1 && gift_index == 0 {
            print!("and ");
        }
        println!("{}", gifts[gift_index])
    }
    }
    

}

