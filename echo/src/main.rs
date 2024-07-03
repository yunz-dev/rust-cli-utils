use std::env;

fn main() {
    // collects arguments into a string vector
    let args:Vec<String> = env::args().collect();
    if args.len() > 1 {
        print!("{}", args[1]);
        if args.len() > 2 {
            for arg in args.iter().skip(2){
                print!(" {arg}");
            }
        }
    }
}
