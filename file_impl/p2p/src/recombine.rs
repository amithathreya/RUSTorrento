use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use serde::Deserialize;
use sha2::{Sha256, Digest};
use anyhow::{Result, Context};

// Metadata structure (same as in split.rs)
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Metadata {
    pub files: Vec<FileMetadata>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FileMetadata {
    pub file_name: String,
    pub chunks: Vec<ChunkMetadata>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChunkMetadata {
    pub chunk_name: String,
    pub sha256: String,
}

/// Recombines files from chunks using metadata.
///
/// # Arguments
/// * `chunk_dir` - The directory containing the chunks and metadata.
/// * `output_dir` - The directory where the recombined files will be saved.
///
/// # Returns
/// * `Result<()>` - Returns `Ok(())` if successful, or an error if something goes wrong.
pub fn recombine_files_from_metadata(chunk_dir: &str, output_dir: &str) -> Result<()> {
    // Read metadata from the JSON file
    let metadata_file = Path::new(chunk_dir).join("metadata.json");
    let metadata_json = fs::read_to_string(&metadata_file)
        .with_context(|| format!("Failed to read metadata file: {}", metadata_file.display()))?;
    let metadata: Metadata = serde_json::from_str(&metadata_json)
        .with_context(|| format!("Failed to parse metadata JSON: {}", metadata_json))?;

    // Ensure the output directory exists
    let output_dir = Path::new(output_dir);
    if !output_dir.exists() {
        fs::create_dir(output_dir).context("Failed to create output directory")?;
    }

    for file_metadata in metadata.files {
        let output_path = output_dir.join(&file_metadata.file_name);
        let mut op_file = File::create(&output_path)
            .with_context(|| format!("Failed to create output file: {}", output_path.display()))?;

        println!("Recombining file: {}", file_metadata.file_name);

        for chunk in &file_metadata.chunks {
            let chunk_path = Path::new(chunk_dir).join(&chunk.chunk_name);
            let mut chunk_file = File::open(&chunk_path)
                .with_context(|| format!("Failed to open chunk file: {}", chunk_path.display()))?;
            let mut buffer = Vec::new();
            chunk_file.read_to_end(&mut buffer)
                .with_context(|| format!("Failed to read chunk file: {}", chunk.chunk_name))?;

            // Verify the hash of the chunk
            let mut hasher = Sha256::new();
            hasher.update(&buffer);
            let calculated_hash = format!("{:x}", hasher.finalize());
            if calculated_hash != chunk.sha256 {
                return Err(anyhow::anyhow!(
                    "Hash mismatch for chunk: {}. Expected: {}, Found: {}",
                    chunk.chunk_name,
                    chunk.sha256,
                    calculated_hash
                ));
            }

            op_file.write_all(&buffer)
                .with_context(|| format!("Failed to write to output file: {}", file_metadata.file_name))?;
        }

        println!("Successfully recombined: {}", file_metadata.file_name);
    }

    Ok(())
}