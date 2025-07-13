enum Type {
    File(String),
    Folder(String)
}

fn main() {
    let type1: Type = Type::File(String::from("report.csv"));
    
    match type1 {
        Type::File(name) => {
            println!("This Is File With Name: [{}]!", name);
        },
        Type::Folder(name) => {
            println!("This Is Folder With Name: [{}]!", name);
        },
    };
}