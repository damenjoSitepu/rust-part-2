fn main() {
    let http_status_code_in_number: i32 = 300;
    let result: &str = match http_status_code_in_number {
        200..=299 => "Success!",
        300..=399 => "Modified!",
        400..=499 => "Error Request!",
        500..=599 => "Internal Server Error!",
        _ => "Unidentified Http Status Code In Number!"
    };
    
    println!("Status Code: {}", result);
}
