use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use serde::{Serialize,Deserialize};
use serde_json;
use sha2::{Sha256, Digest};
use p2p::config;

#[derive(Serialize,Deserialize)] 
pub struct Metadata {
    pub Files:Vec<FileMetaData>,
}
#[derive(Serialize,Deserialize)]
pub struct FileMetaData {
    pub FileName:String,
    pub FileSize:u64,
    pub Chunks:Vec<ChunkMetaData>,
}
#[derive(Serialize,Deserialize)]
pub struct ChunkMetaData {
    pub ChunkName:String,
    pub Sha256:String,
}

pub fn split_file_by_size(dir_path: String, chunk_size: usize)->io::Result<()> {
    let mut metadata = Metadata{Files:Vec::new()};
    // Ensure the output directory exists
    let output_dir = Path::new(config::CHUNK_DATA_PATH);
    if !output_dir.exists() {
        fs::create_dir(output_dir)?;
    }

    for file in fs::read_dir(dir_path)? {
        let file = file?;
        let file_name = file.file_name().to_string_lossy().to_string();
        let file_path = file.path();
        if file_path.is_file() {
            let mut ip_file = File::open(file_path)?;
            let mut buff = vec![0; chunk_size];
            let mut chunks = Vec::new();
            let mut current_chunk = 0;
            loop {
                let bytes_read = ip_file.read(&mut buff)?;
                if bytes_read == 0 {
                    break;
                }   
                let mut hash = Sha256::new();
                hash.update(buff[..bytes_read].as_ref());
                let chunk_hash = format!("{:x}",hash.finalize());

                let chunk_name = format!("{}CHUNK_{}.bin",file_name,current_chunk);
                let chunk_path = output_dir.join(&chunk_name);
                let mut chunk_file = File::create(&chunk_path)?;
                let mut op_file = File::create(&chunk_path)?;
                op_file.write_all(&buff[..bytes_read])?;

                chunks.push(ChunkMetaData{
                    ChunkName:chunk_name,
                    Sha256:chunk_hash,
                });
                current_chunk += 1;
            } 
            metadata.Files.push(FileMetaData{
                FileName:file_name,
                FileSize:ip_file.metadata()?.len(),
                Chunks:chunks,
            });
        }
    }
    let metadata_file = output_dir.join("metadata.json");
    let mut metadata_file = File::create(&metadata_file)?;
    let metadata_json = serde_json::to_string(&metadata)?;
    metadata_file.write_all(metadata_json.as_bytes())?;
    Ok(())
}
