use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let test_data = &args[1];
    let ans_dir = &args[2];
    let sample_number = &args[3];

    let file_dir_name = String::from(ans_dir) + "/";
    let file_name = String::from(sample_number) + ".ipynb";
    let filename = String::from(file_dir_name) + &file_name;
    
    println!("file: {}", filename);
    println!("Test: {}", test_data);

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    println!("{}", contents);

}