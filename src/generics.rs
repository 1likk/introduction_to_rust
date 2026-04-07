fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let number_list = vec![34, 50, 42, 60, 78, 90];

    let result = largest(&number_list);
    println!("The largest number is: {result}");

    let char_list = vec!['y', 'a', 'c', 'b', 'z'];

    let result = largest(&char_list);
    println!("The largest char is: {result}");
}

/* 
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
*/

// Generic data types to duplicat functions 