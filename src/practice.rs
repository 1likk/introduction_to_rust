enum Category {
    fproduct(f64),
    dproduct(String),
}

struct Product {
    name: String,
    price: f64,
    stock: u32,
    category: Category,
}

impl Product {
    fn update_cost(&mut self) -> f64 {
        self.price = self.price - (self.price * 0.10);
        self.price
    }
}

fn main() {
    let mut product1 = Product {
        name: String::from("chocalate"),
        price: 135.5,
        stock: 100,
    };

    let result = find_product(2);

    match result {
        Some(name) => println!("We have that: {}.", name),
        None => println!("We haven't that sorry!"),
    }

    println!("A new cost of chocalate with discount: {}.", product1.update_cost())
}

fn print_info(category: Category) {
    match category {
        Category::fproduct(weight) => {
            println!("This is a physical product, weight {}.", weight);
        },
        Category::dproduct(link) => {
            println!("This is a digital product,link {}.", link);
        },
    }
}


fn find_product(id: u32) -> Option<String> {
    if id == 1 {
        Some(String::from("chocolate"))
    } else {
        None
    }
}