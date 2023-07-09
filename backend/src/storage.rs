pub enum EntryError
{
    IllegalEntryType,
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
    pub fn new(path: EntryPath) -> Result<File, EntryError>
    {
        return match path.entry_type
        {
            EntryType::File => Ok(File { path: path, content: FileContent::NULL }),
            EntryType::Dir => Err(EntryError::IllegalEntryType),
        }
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

impl Dir
{
    pub fn new(path: EntryPath) -> Result<Dir, EntryError>
    {
        return match path.entry_type
        {
            EntryType::File => Err(EntryError::IllegalEntryType),
            EntryType::Dir => Ok(Dir { path: path, entries: DirEntries::NULL }),
        }
    }

    pub fn read(&mut self, storage: &Storage) -> Result<(), std::io::Error>
    {
        return match storage.read_dir(&self.path.path) {
            Ok(read_dir) =>
            {
                let mut entries = Vec::<EntryPath>::new();
                for it in read_dir
                {
                    match it {
                        Ok(entry) =>
                        {
                            let entry_path = entry.path();
                            if entry_path.is_dir() { entries.push(EntryPath { path: storage.convert_to_storage_path(&entry_path).unwrap(), entry_type: EntryType::Dir }) }
                            else { entries.push(EntryPath { path: storage.convert_to_storage_path(&entry_path).unwrap(), entry_type: EntryType::File }) }
                        },
                        Err(err) => return Err(err),
                    }
                }
                self.entries = DirEntries::Content(entries);
                return Ok(());
            },
            Err(err) => Err(err),
        }
    }

    pub fn create(&mut self, storage: &Storage) -> Result<(), std::io::Error>
    {
        return storage.create_dir(&self.path.path);
    }
}


pub struct Storage
{
    path: std::path::PathBuf,
}

impl Storage
{
    pub fn init() -> Result<Storage, std::io::Error>
    {
        let storage_path_string = std::env::var("HOME_DOCK_STORAGE_PATH").unwrap();
        let storage_path = std::path::Path::new(&storage_path_string);
        return match storage_path.exists()
        {
            true => Ok( Storage { path: storage_path.to_path_buf() } ),
            false => match std::fs::create_dir(storage_path) 
            {
                Ok(_) => Ok( Storage { path: storage_path.to_path_buf() } ),
                Err(err) => Err(err),
            }
        }
    }

    pub fn convert_from_storage_path(&self, storage_path: &std::path::PathBuf) -> std::path::PathBuf
    {
        let mut file_path = self.path.clone();
        file_path.push(storage_path);
        return file_path;
    }

    pub fn convert_to_storage_path(&self, path: &std::path::PathBuf) -> Result<std::path::PathBuf, std::path::StripPrefixError>
    {
        return match path.strip_prefix(&self.path)
        {
            Ok(result) => Ok(result.to_path_buf()),
            Err(err) => Err(err),
        }
    }

    pub fn read(&self, storage_path: &std::path::PathBuf) -> Result<Vec<u8>, std::io::Error>
    {
        return std::fs::read(self.convert_from_storage_path(storage_path));
    }

    pub fn write(&self, storage_path: &std::path::PathBuf, content: &Vec<u8>) -> Result<(), std::io::Error>
    {
        return std::fs::write(self.convert_from_storage_path(storage_path), content);
    }

    pub fn read_dir(&self, storage_path: &std::path::PathBuf) -> Result<std::fs::ReadDir, std::io::Error>
    {
        return std::fs::read_dir(self.convert_from_storage_path(storage_path));
    }

    pub fn create_dir(&self, storage_path: &std::path::PathBuf) -> Result<(), std::io::Error>
    {
        let path = self.convert_from_storage_path(storage_path);
        if storage_path.exists() { return Ok(()); }
        return match std::fs::create_dir(path) {
            Ok(()) => Ok(()),
            Err(err) => Err(err),
        }
    }
}