extern crate core;


use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::io::ErrorKind;

fn main() {
    let path = Path::new("hello.txt");
    let mut file = match File::open(path) {
        Ok(file) => { file }
        Err(error) => {
            match error.kind() {
                ErrorKind::NotFound => {
                    File::create(path).unwrap()
                }
                _ => panic!("file open error{}", error)
            }
        }
    };
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    println!("{}", s);
}
