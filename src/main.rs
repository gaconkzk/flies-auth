use std::env::var_os;

fn main() {
    match var_os("FA_PORT") {
        Some(port) => println!("Running flies auth using port {:?}", port),
        None => println!("Port not found in environment, use default: 8088")
    }
}
