use std::env;

fn main() {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => println!("error: {}", e),
    }
}
