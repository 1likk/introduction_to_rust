pub trait Summary {
    fn summarize(&self) -> String;
}

pub trait Quantity {
    fn amounting(&self) -> u32;
}

pub struct Flower {
    name: String,
    count: u32,
}

pub struct Warehouse {
    total_stock: f64,
}

impl Summary for Flower {
    fn summarize(&self) -> String {
        format!("Flower: {}, in stock: {} pc.", self.name, self.count)
    }
}

impl Quantity for Flower {
    fn amounting(&self) -> u32 {
        self.count 
    }
}

impl Quantity for Warhouse {
    fn amounting(&self) -> u32 {
        self.total_stock as u32
    }
}



/* 
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String, 
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
*/