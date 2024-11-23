use async_trait::async_trait; // For async trait methods
use bytes::Bytes; // For reading/writing byte streams
use std::path::PathBuf; // For file paths
use std::sync::Arc;

#[async_trait]
pub trait StorageTrait: Send + Sync {
    async fn get(&self, path: &PathBuf) -> Result<Option<Bytes>, Box<dyn std::error::Error + Send + Sync>>;
    async fn put(&self, path: &PathBuf, data: &Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn delete(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn size(&self, path: &PathBuf) -> Result<Option<u64>, Box<dyn std::error::Error + Send + Sync>>;
    async fn get_files(&self, path: &PathBuf) -> Result<Option<Vec<String>>, Box<dyn std::error::Error + Send + Sync>>;
}

#[derive(Clone)]
pub struct StorageManager {
    storage: Arc<dyn StorageTrait>, // Dynamically dispatched storage implementation
}

impl StorageManager {
    pub fn new(storage: Arc<dyn StorageTrait>) -> Self {
        Self { storage }
    }

    /// Retrieve a file from the given location
    ///
    /// `path` can't be absolute or it will override the base path.
    pub async fn get(&self, path: &PathBuf) -> Result<Option<Bytes>, Box<dyn std::error::Error + Send + Sync>> {
        self.storage.get(path).await
    }

    /// Writes a file to the given location
    ///
    /// `path` can't be absolute or it will override the base path.
    pub async fn put(&self, path: &PathBuf, data: &Vec<u8>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.storage.put(path, data).await
    }

    /// Deletes a file at the given location
    ///
    /// `path` can't be absolute or it will override the base path.
    pub async fn delete(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.storage.delete(path).await
    }

    pub async fn size(&self, path: &PathBuf) -> Result<Option<u64>, Box<dyn std::error::Error + Send + Sync>> {
        self.storage.size(path).await
    }

    pub async fn get_files(&self, path: &PathBuf) -> Result<Option<Vec<String>>, Box<dyn std::error::Error + Send + Sync>> {
        self.storage.get_files(path).await
    }}
