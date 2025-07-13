fn main() {
    let http_status_code: &str = "HTTP_CREATED";
    
    match http_status_code {
        "HTTP_NOT_FOUND" | "HTTP_BAD_REQUEST" => {
            println!("Internal Server Error: 400");
        },
        _ => {
            println!("Unidentified Http Status Code!");
        }
    }
    
    let http_status_code_in_number: i32 = 600;
    
    match http_status_code_in_number {
        200..=299 => {
            println!("Success!");
        },
        300..=399 =>  {
            println!("Modified!");
        },
        400..=499 => {
            println!("Error Request!");
        },
        500..=599 => {
            println!("Internal Server Error!");
        },
        _ => {
            println!("Unidentified Http Status Code In Number!");
        }
    }
}
