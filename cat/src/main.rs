use std::fs::File;
use std::io::prelude::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        for arg in args.iter().skip(1) {
            let mut file = File::open("{arg}");
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            print!("{}", contents)
        }
    }
    
}
