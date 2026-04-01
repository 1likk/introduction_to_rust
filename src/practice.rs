struct Product {
    name: String,
    price: f64,
    stock: u32,
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

    println!("A new cost of chocalate with discount: {}.", product1.update_cost())
}

