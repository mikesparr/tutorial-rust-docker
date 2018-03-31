use std::env;

fn main() {
    let key = "RUST_DOCKER_NAME";

    // print Hello World if ENV name not present
    match env::var(key) {
        Ok(val) => println!("Hello, {}", val),
        _ => println!("Hello World"),
    }
}
