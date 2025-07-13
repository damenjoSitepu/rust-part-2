pub struct Product {
    pub id: u64,
    pub name: String,
    pub price: f32,
    pub qty: u32,
}

impl Product {
    pub fn get_description(&self) -> String {
        format!("Product Name: {} And Their Total Price Is: {}", self.name, self.get_total_price())
    }

    pub fn get_total_price(&self) -> f32 {
        self.price * self.qty as f32
    }
}