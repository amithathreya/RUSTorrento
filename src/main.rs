use std::fs::{self, File};
use std::io::{self, Read, Write};
mod split;
mod config;

fn main() -> io::Result<()> {
    let dir_path = String::from("/home/kazuki/p2p/data/CP");
    let chunk_size = 10 * 1024; // 10 KB
    match split::split_file_by_size(dir_path, chunk_size) {
        Ok(()) => {println!("File split successfully");}
        Err(e) => eprintln!("Error: {}", e),
    }
    Ok(())
}

