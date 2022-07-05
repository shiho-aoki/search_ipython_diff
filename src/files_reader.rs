use std::fs::{File, read_dir};
use std::io;
use std::io::prelude::*;
use std::path::Path;

pub fn cat_file(path: &Path) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    match f.read_to_string(&mut s){
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

pub fn read_data_from_path(path: &Path) -> io::Result<Vec<String>>{
    Ok(read_dir(path)?
        .filter_map(|entry|{
            let entry = entry.ok()?;
            if entry.file_type().ok()?.is_file() {
                Some(entry.file_name().to_string_lossy().into_owned())
            }else{
                None
            }
        }).collect()
    )
}