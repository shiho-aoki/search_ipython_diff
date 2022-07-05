use std::path::Path;

mod files_reader;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let exec_root = Path::new(&args[1]);
    let ans_root = Path::new(&args[2]);
    
    let file_name = String::from(&args[3]) + ".ipynb";
    let filename = ans_root.join(file_name);

    //read ans file
    match files_reader::cat_file(&filename){
        Err(why) => println!("! {:?}", why.kind()),
        Ok(s) => println!("> {}", s),
    }

    //search test data
    match files_reader::read_data_from_path(&exec_root){
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => for path in paths {
            let exec_path = exec_root.join(&path);
            match files_reader::cat_file(&Path::new(&exec_path)){
                Err(why) => println!("! {:?}", why.kind()),
                Ok(s) => println!("> {}",s),
            }
        },
    }
}