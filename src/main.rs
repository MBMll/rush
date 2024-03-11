use std::{
    env,
    fs::File,
    io::{Read, Write},
    process::exit,
};

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

fn split_file(mut args: Vec<String>) {
    let file_name = args.pop().expect("Not found file name!");
    let mut source_file = File::open(file_name).expect("Not opened file!");
    let mut buff = [0u8, 4096];
    let mut count = 0;
    while source_file.read(&mut buff).unwrap() != 0 {
        count += buff.len();
        let mut file = File::create(file_name + "s").expect("Create file failed!");
        file.write(&buff);
        println!("count: {:?}", count);
    }
}
