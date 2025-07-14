trait Block {
    fn get_default_name(&self) -> String {
        String::from("Default Block Name")
    }

    fn get_default_names(&self) -> String {
        String::from("Default Block Name II")
    }

    fn get_name(&self) -> String;

    fn calculate_rectangle(&self, w: u32, h: u32) -> u32;
}

trait Color {
    fn get_random(&self) -> String;
}

struct Rectangle {
    name: String, 
}

impl Block for Rectangle {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn calculate_rectangle(&self, w: u32, h: u32) -> u32 {
        w * h
    }
}

impl Color for Rectangle {
    fn get_random(&self) -> String {
        String::from("#fff")
    }
}

fn main() {
    let rectangle: Rectangle = Rectangle {
        name: String::from("Smosh"),
    };
    println!("{}, {}", rectangle.name, rectangle.calculate_rectangle(10, 20));

    let random_color: String = rectangle.get_random();
    println!("Random Color: {}", random_color);

    let default_name: String = rectangle.get_default_names();
    println!("Default Name: {}", default_name);

    let get_block_name: String = Block::get_name(&rectangle);
    println!("Get Block Name: {}", get_block_name);
}
