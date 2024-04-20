pub trait RushTask {
    fn run(&self, args: Vec<String>);
}

pub mod split_file;
