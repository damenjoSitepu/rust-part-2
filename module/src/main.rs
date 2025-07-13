pub mod services;
pub mod models;

fn main() {
    services::util::say_hello_world();
    let math_result: i32 = services::math::multiply(2,4);
    println!("Math Result: {}", math_result);

    let product: models::product::Product = models::product::Product {
            id: 1,
            name: String::from("Woody"),
            price: 10.0,
            qty: 3,
        };

    println!("{}", product.get_description());
}