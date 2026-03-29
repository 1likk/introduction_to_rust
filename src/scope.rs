fn scope() {
    let s1 = "external";
    {
        let s2 = "internal";
        println!("{s1} and {s2}")
    }

    println!("{s1}")
}