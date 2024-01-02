use procedural_macros::*;

trait Log {
    fn info(&self, msg: &str);
    fn warn(&self, msg: &str);
    fn error(&self, msg: &str);
}

#[derive(Debug, Log)]
struct Database {
    url: String,
    connections: u32,
}

impl Database {
    fn new(url: String) -> Self {
        Self {
            url,
            connections: 0,
        }
    }

    fn connect(&mut self) {
        self.info(format!("new connection to {}", self.url).as_str());
        self.connections += 1;

        if self.connections > 100 {
            self.warn(format!("100 more connections open!").as_str());
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Product {
    name: String,
    price: u32,
}

fn main() {
    log_info!([TIME] starting program...);

    let mut db = Database::new("localhost:8080".to_string());

    for _ in 0..=100 {
        db.connect();
    }

    let laptop = Product {
        name: "Dell".to_string(),
        price: 2000,
    };
    by_product(laptop, 20);
}

#[log_call(verbose)]
fn by_product(product: Product, discount: u32) {
    println!("[Info] calling by_product | product = {product:?} discount = {discount}");
}
