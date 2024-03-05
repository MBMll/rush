use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let mut args = args.clone();
    args.reverse();
    let func = args.pop();
    let func = args.pop();
    dbg!(&func);
    match func {
        Some(func) => {
            if "split".eq(&func) {
                split_file(args);
            }
        }
        None => {
            println!("Least 2 arguments are needed.")
        }
    }
}

fn split_file(args: Vec<String>) {
    dbg!(args);
}
