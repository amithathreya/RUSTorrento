use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    let dir_path = String::from("/home/kazuki/p2p/data/CP");
    let chunk_size = 10 * 1024; // 10 KB
    match split_file_by_size(dir_path, chunk_size) {
        Ok(()) => println!("File split successfully"),
        Err(e) => eprintln!("Error: {}", e),
    }
    Ok(())
}

fn split_file_by_size(dir_path: String, chunk_size: usize) -> io::Result<()> {
    let mut current_chunk = 0;
    let mut current_chunk_size = 0;

    // Ensure the output directory exists
    let output_dir = Path::new("data");
    if !output_dir.exists() {
        fs::create_dir(output_dir)?;
    }

    let mut op_file = File::create(output_dir.join(format!("chunk_{}.bin", current_chunk)))?;

    for file in fs::read_dir(dir_path)? {
        let file = file?;
        let file_path = file.path();
        if file_path.is_file() {
            let mut ip_file = File::open(file_path)?;
            let mut buff = vec![0; chunk_size];
            loop {
                let bytes_read = ip_file.read(&mut buff)?;
                if bytes_read == 0 {
                    break;
                }   
                let mut start = 0;
                while start < bytes_read {
                    let remaining = bytes_read - start;
                    let space_left = chunk_size - current_chunk_size;

                    if remaining <= space_left {
                        op_file.write_all(&buff[start..start + remaining])?;
                        current_chunk_size += remaining;
                        start += remaining;
                    } else {
                        op_file.write_all(&buff[start..start + space_left])?;
                        current_chunk += 1;
                        current_chunk_size = 0;
                        op_file = File::create(output_dir.join(format!("chunk_{}.bin", current_chunk)))?;
                        start += space_left;
                    }
                }
            }
        }
    }
    Ok(())
}
