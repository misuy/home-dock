mod storage;

#[macro_use] extern crate rocket;


#[post("/read_file", data = "<path>")]
fn read_file(storage: &rocket::State<storage::Storage>, path: rocket::serde::json::Json<storage::EntryPath>) -> Result<rocket::serde::json::Json<storage::File>, std::io::Error>
{
    let mut file = storage::File::new(path.into_inner());
    return match file.read(storage)
    {
        Ok(()) => Ok(rocket::serde::json::Json(file)),
        Err(err) => Err(err),
    }
}


#[post("/write_file", data = "<file>")]
fn write_file(storage: &rocket::State<storage::Storage>, file: rocket::serde::json::Json<storage::File>) -> ()
{
    file.write(storage);
}




#[launch]
fn launch() -> rocket::Rocket<rocket::Build>
{
    let storage = storage::Storage::init().unwrap();
    rocket::build().manage(storage).mount("/home_dock_api", routes![read_file, write_file])
}
