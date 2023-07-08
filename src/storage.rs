use std::fs::ReadDir;

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
pub enum EntryType
{
    File,
    Dir,
}

#[derive(rocket::serde::Serialize, rocket::serde::Deserialize)]
pub struct EntryPath
{
    path: std::path::PathBuf,
    entry_type: EntryType,
}


#[derive(rocket::serde::Serialize, rocket::serde::Deserialize)]
pub enum FileContent {
    Content(Vec<u8>),
    NULL,
}

#[derive(rocket::serde::Serialize, rocket::serde::Deserialize)]
pub struct File
{
    path: EntryPath,
    content: FileContent,
}


impl File
{
    pub fn new(path: EntryPath) -> File
    {
        return File { path: path, content: FileContent::NULL };
    }

    pub fn read(&mut self, storage: &Storage) -> Result<(), std::io::Error>
    {
        return match storage.read(&self.path.path) {
            Ok(content) =>
            {
                self.content = FileContent::Content(content);
                Ok(())
            },
            Err(err) => Err(err),
        }
    }

    pub fn write(&self, storage: &Storage) -> Result<bool, std::io::Error>
    {
        let content = &self.content;
        return match content
        {
            FileContent::Content(content) => 
            {
                match storage.write(&self.path.path, content)
                {
                    Ok(()) => Ok(true),
                    Err(err) => Err(err),
                }
            }
            FileContent::NULL => Ok(false),
        }
    }
}


#[derive(rocket::serde::Serialize, rocket::serde::Deserialize)]
pub enum DirEntries {
    Content(Vec<EntryPath>),
    NULL,
}

#[derive(rocket::serde::Serialize, rocket::serde::Deserialize)]
pub struct Dir
{
    path: EntryPath,
    entries: DirEntries,
}


impl Storage
{
    pub fn init() -> Result<Storage, std::io::Error>
    {
        let storage_path_string = std::env::var("HOME_DOCK_STORAGE_PATH").unwrap();
        let storage_path = std::path::Path::new(&storage_path_string);
        return match storage_path.exists()
        {
            true => Ok( Storage { path_buf: storage_path.to_path_buf() } ),
            false => match std::fs::create_dir(storage_path) 
            {
                Ok(i) => Ok( Storage { path_buf: storage_path.to_path_buf() } ),
                Err(err) => Err(err),
            }
        }
    }

    pub fn convert_storage_path(&self, storage_path: &std::path::PathBuf) -> std::path::PathBuf
    {
        let mut file_path = self.path_buf.clone();
        file_path.push(storage_path);
        return file_path;
    }

    pub fn read(&self, storage_path: &std::path::PathBuf) -> Result<Vec<u8>, std::io::Error>
    {
        return std::fs::read(self.convert_storage_path(storage_path));
    }

    pub fn write(&self, storage_path: &std::path::PathBuf, content: &Vec<u8>) -> Result<(), std::io::Error>
    {
        return std::fs::write(self.convert_storage_path(storage_path), content);
    }

    pub fn read_dir(&self, storage_path: &std::path::PathBuf) -> Result<ReadDir, std::io::Error>
    {
        return std::fs::read_dir(self.convert_storage_path(storage_path));
    }
}