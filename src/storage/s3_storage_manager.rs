use std::env;
use std::error::Error;
use async_trait::async_trait; // For async trait methods
use bytes::Bytes;
use std::path::PathBuf;
use log::info;
use tokio::fs;
use crate::storage::storage_manager::StorageTrait; // Use tokio::fs

//TODO: currently uses a file system implementation, should use S3 in the future
pub struct S3SystemStorage {
    base_dir: PathBuf,
}

impl S3SystemStorage {
    pub fn new(base_dir: String) -> Self {
        let absolute_base_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("/")).join(base_dir); // Fallback to root if current dir not found
        info!("Base dir is: {}", absolute_base_dir.display());

        Self { base_dir: absolute_base_dir }
    }
}

#[async_trait]
impl StorageTrait for S3SystemStorage {
    async fn get(&self, path: &PathBuf) -> Result<Option<Bytes>, Box<dyn Error + Send + Sync>> {
        let full_path = self.base_dir.join(path);
        if let Ok(data) = fs::read(full_path).await {
            Ok(Some(Bytes::from(data)))
        } else {
            Ok(None)
        }
    }

    async fn put(&self, path: &PathBuf, data: &Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync>> {
        info!("Writing {}", path.display());
        let full_path = self.base_dir.join(path);
        if let Some(parent_dir) = full_path.parent() {
            fs::create_dir_all(parent_dir).await?; // Create directories if not present
        }
        fs::write(full_path, data).await?;
        Ok(())
    }

    async fn delete(&self, path: &PathBuf) -> Result<(), Box<dyn Error + Send + Sync>> {
        let full_path = self.base_dir.join(path);
        fs::remove_file(full_path).await?;
        Ok(())
    }

    async fn size(&self, path: &PathBuf) -> Result<Option<u64>, Box<dyn Error + Send + Sync>> {
        let mut total_size: u64 = 0;
        let full_path = self.base_dir.join(path);

        fs::create_dir_all(&full_path).await?; // Create directories if not present
        let mut entries = fs::read_dir(full_path).await?;
        while let Some(entry) = entries.next_entry().await? {
            let metadata = entry.metadata().await?;
            if metadata.is_file() {
                total_size += metadata.len();
            } else if metadata.is_dir() {
                total_size += self.size(&entry.path()).await?.unwrap();
            }
        }

        Ok(Some(total_size))
    }

    async fn get_files(&self, path: &PathBuf) -> Result<Option<Vec<String>>, Box<dyn Error + Send + Sync>> {
        let mut files: Vec<String> = Vec::new();
        let full_path = self.base_dir.join(path);

        let mut entries = match fs::read_dir(full_path).await {
            Ok(dir) => {
                dir
            }
            Err(_) => {
                return Ok(None);
            }
        };
        while let Some(entry) = entries.next_entry().await? {
            files.push(entry.file_name().to_str().unwrap().to_string());
        }

        Ok(Some(files))
    }
}