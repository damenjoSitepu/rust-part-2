
fn main() {
    let airport_name: &str = "La Guardia";
    
    match airport_name {
        "La Guardia" => {
            println!("Welcome To La Guardia Airport!");
        },
        _ => {
            println!("Unidentified Airport!");
        }
    }
}
