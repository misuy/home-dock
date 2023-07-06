#[derive(Debug, PartialEq, Eq, Clone)]
pub enum StorageError
{
    CantCreateStorageDir,
}

pub struct Storage
{
    path_buf: std::path::PathBuf,
}

#[derive(rocket::serde::Serialize, rocket::serde::Deserialize)]
pub struct File
{
    content: Vec<u8>,
}

impl Storage
{
    pub fn init() -> Result<Storage, StorageError>
    {
        let storage_path_string = std::env::var("HOME_DOCK_STORAGE_PATH").unwrap();
        let storage_path = std::path::Path::new(&storage_path_string);
        if !storage_path.exists() { if std::fs::create_dir(storage_path).is_err() { return Err(StorageError::CantCreateStorageDir); } }
        return Ok( Storage { path_buf: storage_path.to_path_buf() } );
    }

    pub fn read_file(&self, storage_file_path: std::path::PathBuf) -> Result<File, std::io::Error>
    {
        let mut file_path = self.path_buf.clone();
        file_path.push(storage_file_path);
        let read_result = std::fs::read(file_path);
        if read_result.is_err() { return Err(read_result.unwrap_err()) };
        return Ok(File { content: read_result.unwrap() });
    }
}