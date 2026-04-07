fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let max = largest(&number_list);
    println!("the largest number of list: {max}")
}

fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    
    println!("Largest number is: {largest}");
    largest
}