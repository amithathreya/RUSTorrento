use std::fs::{self, File};
use std::io::{ Read, Write};
use std::path::Path;
use serde::{Serialize, Deserialize};
use serde_json;
use sha2::{Sha256, Digest};
use anyhow::{Result, Context};
use rayon::prelude::*;


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Metadata {
    pub files: Vec<FileMetadata>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FileMetadata {
    pub file_name: String,
    pub chunks: Vec<ChunkMetadata>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChunkMetadata {
    pub chunk_name: String,
    pub sha256: String,
}

pub fn split_file_by_size(dir_path: String, chunk_size: usize, output_dir: &str) -> Result<()> {

    // Ensure the output directory exists
    let output_dir = Path::new(output_dir);
    if !output_dir.exists() {
        fs::create_dir(output_dir).context("Failed to create output directory")?;
    }

    let files:Vec<_> = fs::read_dir(&dir_path).context("Failed to read input directory")?
    .filter_map(|entry| entry.ok())
    .filter(|entry| entry.path().is_file()).collect();

    let metadata = files
        .par_iter() 
        .map(|file| {
            let file_name = file.file_name().to_string_lossy().to_string();
            let file_path = file.path();

            let mut ip_file = File::open(&file_path)
                .with_context(|| format!("Failed to open file: {}", file_name))?;
            let mut buff = vec![0; chunk_size];
            let mut chunks = Vec::new();
            let mut current_chunk = 0;

            loop {
                let bytes_read = ip_file.read(&mut buff)
                    .with_context(|| format!("Failed to read file: {}", file_name))?;
                if bytes_read == 0 {
                    break;
                }

                // Calculate hash of the chunk
                let mut hasher = Sha256::new();
                hasher.update(&buff[..bytes_read]);
                let chunk_hash = format!("{:x}", hasher.finalize());

                // Save the chunk to a file
                let chunk_name = format!("{}_chunk_{}.bin", file_name, current_chunk);
                let chunk_path = output_dir.join(&chunk_name);
                let mut op_file = File::create(&chunk_path)
                    .with_context(|| format!("Failed to create chunk file: {}", chunk_name))?;
                op_file.write_all(&buff[..bytes_read])
                    .with_context(|| format!("Failed to write to chunk file: {}", chunk_name))?;

                // Add chunk metadata
                chunks.push(ChunkMetadata {
                    chunk_name,
                    sha256: chunk_hash,
                });

                current_chunk += 1;
            }

            // Return file metadata
            Ok(FileMetadata {
                file_name,
                chunks,
            })
        })
        .collect::<Result<Vec<_>>>()?; // Collect results into a Vec<FileMetadata>

    // Save metadata to a JSON file
    let metadata = Metadata { files: metadata };
    let metadata_file = output_dir.join("metadata.json");
    let metadata_json = serde_json::to_string_pretty(&metadata)
        .context("Failed to serialize metadata to JSON")?;
    File::create(&metadata_file)
        .context("Failed to create metadata file")?
        .write_all(metadata_json.as_bytes())
        .context("Failed to write metadata to file")?;

    println!("Files split successfully with metadata.");
    Ok(())
}