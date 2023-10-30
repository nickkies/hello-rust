struct Product {
    name: String,
    price: f32,
    in_stock: bool,
}

impl Product {
    fn new(name: String, price: f32) -> Self {
        Self {
            name,
            price,
            in_stock: true,
        }
    }
    fn get_default_sales_tax() -> f32 {
        0.1
    }
    fn calculate_salse_tax(&self) -> f32 {
        self.price * Self::get_default_sales_tax()
    }
    fn set_price(&mut self, price: f32) {
        self.price = price;
    }
    fn buy(self) -> i32 {
        let name = self.name;
        println!("{name} was bought!");
        123
    }
}

fn main() {
    let mut book = Product::new(String::from("book"), 30.0);
    let sales_tax = book.calculate_salse_tax();

    println!("sales tax is: {sales_tax}");

    book.set_price(20.0);
    book.buy(); // drop book instance

    // println!("{} price is: {}", book.name, book.price);
}
