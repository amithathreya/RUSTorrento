use std::io;
mod split;
mod config;
mod recombine;
fn main() -> io::Result<()> {
    let dir_path = String::from(config::DATA_PATH);
    let chunk_size = 256 * 1024; 
    match split::split_file_by_size(dir_path, chunk_size,config::CHUNK_DATA_PATH) {
        Ok(()) => {println!("File split successfully");}
        Err(e) => eprintln!("Error: {}", e),
    }
    match recombine::recombine_files_from_metadata(config::CHUNK_DATA_PATH, 
        config::RECOMBINE_PATH) {
        Ok(()) =>
            println!("File recombined successfully"),
        Err(e) => eprintln!("Error: {}", e),
    }   
    Ok(())
}

