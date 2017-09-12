#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket_contrib;
extern crate rocket;
#[macro_use] extern crate serde_derive;

use rocket::response::status::NotFound;
use std::path::Path;
use rocket::response::NamedFile;

use rocket_contrib::Json;

use std::{thread, time};

#[get("/")]
fn index() -> Result<NamedFile, NotFound<String>>  {
    let path = Path::new("static/index.html");
   NamedFile::open(&path).map_err(|_| NotFound(String::from("Bad path")))
}

#[get("/scripts/<path_param>")]
fn vue(path_param: String) -> Result<NamedFile, NotFound<String>> {
    let mut combined_path: String = "scripts/".to_string();
    combined_path.push_str(&path_param);
   let path = Path::new(&combined_path);
   NamedFile::open(&path).map_err(|_| NotFound(String::from("Bad path")))
}

#[derive(Serialize)]
struct Serie {
    name: String,
    data: Vec<u32>
}

#[derive(Serialize)]
struct JsonData { 
    series: Vec<Serie>
}

#[get("/chart-data")]
fn jsonsend() -> Json<JsonData> {

    thread::sleep(time::Duration::from_millis(3000));

    let mut data = Vec::new();
    data.push(Serie{name:String::from("Tokyo"), data:[7, 6, 9, 14, 18, 21, 25, 26, 23, 18, 13, 9].to_vec()});
    data.push(Serie{name:String::from("New York"), data: [0, 1, 5, 11, 17, 22, 24, 24, 20, 14, 8, 2].to_vec()});
    data.push(Serie{name:String::from("Berlin"), data:[0, 1, 3, 8, 13, 17, 18, 17, 14, 9, 3, 1].to_vec()});
    data.push(Serie{name:String::from("London"), data: [3, 4, 5, 8, 11, 15, 17, 16, 14, 10, 6, 4].to_vec()});

   Json(JsonData{series:data})
}

fn main() {
    rocket::ignite()
    .mount("/", routes![index])
    .mount("/", routes![vue])
    .mount("/", routes![jsonsend])
}
