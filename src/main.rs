use std::{env, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);
    let mut args = args.clone();
    args.reverse();
    args.pop();
    let func = args.pop();
    // dbg!(&func);
    match func {
        Some(func) => {
            if "split".eq(&func) {
                return split_file(args);
            }
            println!("Not found command: [{}]", func);
            exit(404)
        }
        None => {
            println!("Least 2 arguments are needed.")
        }
    }
}

fn split_file(args: Vec<String>) {
    dbg!(args);
}
