//^^
mod storage;

#[macro_use] extern crate rocket;


#[post("/read_file", data = "<path>")]
fn read_file(storage: &rocket::State<storage::Storage>, path: rocket::serde::json::Json<storage::EntryPath>) -> Result<rocket::serde::json::Json<storage::File>, std::io::Error>
{
    return match storage::File::new(path.into_inner())
    {
        Ok(mut file) => match file.read(storage)
        {
            Ok(()) => Ok(rocket::serde::json::Json(file)),
            Err(err) => Err(err),
        }
        Err(_) => Err(std::io::Error::new(std::io::ErrorKind::Other, "IllegalEntryType")),
    }
}


#[post("/write_file", data = "<file>")]
fn write_file(storage: &rocket::State<storage::Storage>, file: rocket::serde::json::Json<storage::File>) -> Result<(), std::io::Error>
{
    return match file.write(storage)
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}


#[post("/create_dir", data = "<path>")]
fn create_dir(storage: &rocket::State<storage::Storage>, path: rocket::serde::json::Json<storage::EntryPath>) -> Result<(), std::io::Error>
{
    return match storage::Dir::new(path.into_inner())
    {
        Ok(mut dir) => match dir.create(storage)
        {
            Ok(()) => Ok(()),
            Err(err) => Err(err),
        }
        Err(_) => Err(std::io::Error::new(std::io::ErrorKind::Other, "IllegalEntryType")),
    }
}


#[post("/read_dir", data = "<path>")]
fn read_dir(storage: &rocket::State<storage::Storage>, path: rocket::serde::json::Json<storage::EntryPath>) -> Result<rocket::serde::json::Json<storage::Dir>, std::io::Error>
{
    return match storage::Dir::new(path.into_inner())
    {
        Ok(mut dir) => match dir.read(storage)
        {
            Ok(()) => Ok(rocket::serde::json::Json(dir)),
            Err(err) => Err(err),
        }
        Err(_) => Err(std::io::Error::new(std::io::ErrorKind::Other, "IllegalEntryType")),
    }
}


#[launch]
fn launch() -> rocket::Rocket<rocket::Build>
{
    let storage = storage::Storage::init().unwrap();
    rocket::build().manage(storage).mount("/home_dock_api", routes![read_file, write_file, read_dir, create_dir])
}
