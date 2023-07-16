//^^
mod storage;

#[macro_use] extern crate rocket;


#[get("/read_file/<path..>")]
fn read_file(storage: &rocket::State<storage::Storage>, path: std::path::PathBuf) -> storage::FileContent
{
    let mut file = storage::File::new(storage::EntryPath { path: path, entry_type: storage::EntryType::File }).unwrap();
    file.read(storage);
    return file.content;
}


#[post("/write_file/<path..>", data = "<content>")]
fn write_file(storage: &rocket::State<storage::Storage>, path: std::path::PathBuf, content: rocket::serde::json::Json<storage::FileContent>) -> storage::EntryType
{
    let file = storage::File::new_with_content(storage::EntryPath { path: path, entry_type: storage::EntryType::File }, content.into_inner().content).unwrap();
    return match file.write(storage)
    {
        Ok(_) => storage::EntryType::File,
        Err(_) => storage::EntryType::NULL,
    }
}


#[post("/create_dir/<path..>")]
fn create_dir(storage: &rocket::State<storage::Storage>, path: std::path::PathBuf) -> storage::EntryType
{
    return match storage::Dir::new(storage::EntryPath { path: path, entry_type: storage::EntryType::Dir })
    {
        Ok(mut dir) => match dir.create(storage)
        {
            Ok(()) => storage::EntryType::Dir,
            Err(_) => storage::EntryType::NULL,
        }
        Err(_) => storage::EntryType::NULL,
    }
}


#[get("/read_dir/<path..>")]
fn read_dir(storage: &rocket::State<storage::Storage>, path: std::path::PathBuf) -> storage::DirEntries
{
    let mut dir = storage::Dir::new(storage::EntryPath { path: path, entry_type: storage::EntryType::Dir }).unwrap();
    dir.read(storage);
    return dir.entries;
}


#[get("/check_entry_type/<path..>")]
fn check_entry_type(storage: &rocket::State<storage::Storage>, path: std::path::PathBuf) -> storage::EntryType
{
    let mut entry_path = storage::EntryPath { path: path, entry_type: storage::EntryType::NULL };
    entry_path.check_entry_type(storage);
    return entry_path.entry_type;
}


#[post("/remove_entry/<path..>")]
fn remove_entry(storage: &rocket::State<storage::Storage>, path: std::path::PathBuf) -> storage::EntryType
{
    let mut entry_path = storage::EntryPath { path: path, entry_type: storage::EntryType::NULL };
    entry_path.remove_entry(storage);
    entry_path.check_entry_type(storage);
    return entry_path.entry_type;
}


#[launch]
fn launch() -> rocket::Rocket<rocket::Build>
{
    let storage = storage::Storage::init().unwrap();
    rocket::build().manage(storage).mount("/home_dock_api", routes![read_file, write_file, read_dir, create_dir, check_entry_type, remove_entry])
}
