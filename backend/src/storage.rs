#[derive(std::fmt::Debug)]
pub enum EntryError
{
    IllegalEntryType,
}


pub enum EntryType
{
    File,
    Dir,
}

impl EntryType
{
    fn to_json(self) -> String
    {
        return match self {
            EntryType::File => "file".to_string(),
            EntryType::Dir => "dir".to_string(),
        }
    }
}


pub struct EntryPath
{
    pub path: std::path::PathBuf,
    pub entry_type: EntryType,
}

impl EntryPath
{
    fn to_json(self) -> String
    {
        let mut json = "{".to_string();
        json.push_str("\"path\":\"");
        json.push_str(self.path.to_str().unwrap());
        json.push_str("\",\"entry_type\":\"");
        json.push_str(self.entry_type.to_json().as_str());
        json.push_str("\"}");
        return json;
    }
}


#[derive(rocket::serde::Deserialize)]
pub struct FileContent
{
    pub content: Vec<u8>,
}

impl FileContent
{
    pub fn to_json(self) -> String
    {
        let mut json = "[".to_string();
        for el in self.content
        {
            json.push_str(el.to_string().as_str());
            json.push(',');
        }
        if json.ends_with(',') { json.pop(); }
        json.push(']');
        return json;
    }
}

impl<'r> rocket::response::Responder<'r, 'static> for FileContent
{

    fn respond_to(self, request:  &'r rocket::Request<'_>) -> rocket::response::Result<'static>
    {
        let json = self.to_json();
        rocket::Response::build()
            .streamed_body(std::io::Cursor::new(json))
            .raw_header("Access-Control-Allow-Origin", "*")
            .header(rocket::http::ContentType::new("application", "json"))
            .ok()
    }
}


pub struct File
{
    path: EntryPath,
    pub content: FileContent,
}


impl File
{
    pub fn new(path: EntryPath) -> Result<File, EntryError>
    {
        return match path.entry_type
        {
            EntryType::File => Ok(File { path: path, content: FileContent{ content: Vec::new() } }),
            EntryType::Dir => Err(EntryError::IllegalEntryType),
        }
    }

    pub fn new_with_content(path: EntryPath, content: Vec<u8>) -> Result<File, EntryError>
    {
        return match path.entry_type
        {
            EntryType::File => Ok(File { path: path, content: FileContent{ content: content } }),
            EntryType::Dir => Err(EntryError::IllegalEntryType),
        }
    }

    pub fn read(&mut self, storage: &Storage) -> Result<(), std::io::Error>
    {
        return match storage.read(&self.path.path) {
            Ok(content) =>
            {
                self.content = FileContent{ content: content };
                Ok(())
            },
            Err(err) => Err(err),
        }
    }

    pub fn write(&self, storage: &Storage) -> Result<bool, std::io::Error>
    {
        let content = &self.content;
        return match storage.write(&self.path.path, &content.content)
        {
            Ok(()) => Ok(true),
            Err(err) => Err(err),
        }
    }

    pub fn to_json(self) -> String
    {
        let mut json = "{".to_string();
        json.push_str("\"path\":\"");
        json.push_str(self.path.to_json().as_str());
        json.push_str("\",\"content\":");
        json.push_str(self.content.to_json().as_str());
        json.push_str("}");
        return json;
    }

    
}

impl<'r> rocket::response::Responder<'r, 'static> for File
{

    fn respond_to(self, request:  &'r rocket::Request<'_>) -> rocket::response::Result<'static>
    {
        let json = self.to_json();
        rocket::Response::build()
            .streamed_body(std::io::Cursor::new(json))
            .raw_header("Access-Control-Allow-Origin", "*")
            .header(rocket::http::ContentType::new("application", "json"))
            .ok()
    }
}


pub struct DirEntries
{
    entries: Vec<EntryPath>,
}

impl DirEntries
{
    pub fn to_json(self) -> String
    {
        let mut json = "[".to_string();
        for entry in self.entries
        {
            json.push_str(entry.to_json().as_str());
            json.push(',');
        }
        if json.ends_with(',') { json.pop(); }
        json.push(']');
        return json;
    }
}

impl<'r> rocket::response::Responder<'r, 'static> for DirEntries
{

    fn respond_to(self, request:  &'r rocket::Request<'_>) -> rocket::response::Result<'static>
    {
        let json = self.to_json();
        rocket::Response::build()
            .streamed_body(std::io::Cursor::new(json))
            .raw_header("Access-Control-Allow-Origin", "*")
            .header(rocket::http::ContentType::new("application", "json"))
            .ok()
    }
}


pub struct Dir
{
    path: EntryPath,
    pub entries: DirEntries,
}

impl Dir
{
    pub fn new(path: EntryPath) -> Result<Dir, EntryError>
    {
        return match path.entry_type
        {
            EntryType::File => Err(EntryError::IllegalEntryType),
            EntryType::Dir => Ok(Dir { path: path, entries: DirEntries { entries: Vec::new() } }),
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
                self.entries = DirEntries { entries: entries };
                return Ok(());
            },
            Err(err) => Err(err),
        }
    }

    pub fn create(&mut self, storage: &Storage) -> Result<(), std::io::Error>
    {
        return storage.create_dir(&self.path.path);
    }

    fn to_json(self) -> String
    {
        let mut json = "{".to_string();
        json.push_str("\"path\":\"");
        json.push_str(self.path.to_json().as_str());
        json.push_str("\",\"entries\":");
        json.push_str(self.entries.to_json().as_str());
        json.push_str("}");
        return json;
    }
}

impl<'r> rocket::response::Responder<'r, 'static> for Dir
{

    fn respond_to(self, request:  &'r rocket::Request<'_>) -> rocket::response::Result<'static>
    {
        let json = self.to_json();
        rocket::Response::build()
            .streamed_body(std::io::Cursor::new(json))
            .raw_header("Access-Control-Allow-Origin", "*")
            .header(rocket::http::ContentType::new("application", "json"))
            .ok()
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