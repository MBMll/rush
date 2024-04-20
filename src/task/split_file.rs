use std::{
    fs::File,
    io::{Read, Seek, SeekFrom, Write},
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};
// use task::RushTask;
use crate::task::RushTask;
pub struct SplitFile {}

// crate::task::RushTask;

impl RushTask for SplitFile {
    fn run(&self, mut args: Vec<String>) {
        // parse arguments
        println!("{:?}", args);
        let source_file = args.pop().expect("Not found file name!");
        println!("{:?}", source_file);
        let num_threads = args
            .pop()
            .unwrap_or_else(|| String::from(""))
            .parse::<i32>()
            .unwrap_or_else(|_| 5);

        let chunk_size = args
            .pop()
            .unwrap_or_else(|| String::from(""))
            .parse::<u64>()
            .unwrap_or_else(|_| 500);

        split_file_multiple_thread(
            CopyParams {
                source_file,
                destination_file: "".to_owned(),
            },
            num_threads,
            1014 * 1024 * chunk_size,
        )
    }
}

fn split_file_multiple_thread(sd: CopyParams, num_threads: i32, chunk_size: u64) {
    let file = File::open(sd.source_file).expect("Cannot read the file");
    let file_length = file.metadata().unwrap().len() as usize;
    let file_count = (file_length as f64 / chunk_size as f64).ceil() as i32;
    let arc_file = Arc::new(Mutex::new(file));
    let actual_num_threads = if file_count > num_threads {
        num_threads
    } else {
        file_count
    };
    let mut threads: Vec<JoinHandle<()>> = Vec::with_capacity(actual_num_threads as usize);
    let destination_file = Arc::new(sd.destination_file);
    for i in 0..actual_num_threads {
        let arc_file = Arc::clone(&arc_file);
        let destination_file = Arc::clone(&destination_file);
        let spawn = thread::spawn(move || {
            let file = arc_file.lock().unwrap();
            copy(
                &file,
                &format!("{}.{}", destination_file, i),
                i as u64 * chunk_size as u64,
                chunk_size,
            )
        });
        threads.push(spawn);
    }
    for i in threads {
        i.join().unwrap();
    }
}
pub struct CopyParams {
    source_file: String,
    destination_file: String,
}

pub fn copy(mut file: &File, target_path: &String, offset: u64, size: u64) {
    let mut buffer = [0; 4096];
    let mut bytes_read = 0;
    file.seek(SeekFrom::Start(offset)).unwrap();
    let mut destination = File::create(target_path).expect("create file failed!");
    while bytes_read < size {
        if let Result::Ok(len) = file.read(&mut buffer) {
            let mut len = len as u64;
            if len > 0 {
                // limit max of size
                if bytes_read + len > size {
                    len = size - bytes_read;
                }
                // recursive the value
                bytes_read += &len;
                destination
                    .write_all(&buffer[..len as usize])
                    .expect("write to file failed!");
                continue;
            }
        }
        break;
    }
}
