mod storage;

#[macro_use] extern crate rocket;


#[get("/read_file/<path..>")]
fn read_file(storage: &rocket::State<storage::Storage>, path: std::path::PathBuf) -> rocket::serde::json::Json<storage::File>
{
    return rocket::serde::json::Json(storage.read_file(path).unwrap());
}


#[post("/write_file/<path..>")]
fn write_file(storage: &rocket::State<storage::Storage>, path: std::path::PathBuf) -> ()
{
    
}


#[launch]
fn launch() -> rocket::Rocket<rocket::Build>
{
    let storage = storage::Storage::init().unwrap();
    rocket::build().manage(storage).mount("/home_dock_api", routes![read_file])
}
