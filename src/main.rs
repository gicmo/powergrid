#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rusqlite;

#[macro_use]
extern crate rocket_contrib;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::io;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use rocket::response::{NamedFile, Responder, Response};
use rocket::http;
use rocket::State;
use rusqlite::Connection;

type DB = Mutex<Connection>;

#[derive(Debug)]
enum Error {
    Internal(String),
}

impl From<rusqlite::Error> for Error {
    fn from(error: rusqlite::Error) -> Error {
        match error {
            _ => Error::Internal("SQL error".into()),
        }
    }
}

impl<'r> Responder<'r> for Error {
    fn respond(self) -> Result<Response<'r>, http::Status> {
        let mut builder = Response::build();
        builder.header(http::ContentType::JSON);

        match self {
            Error::Internal(val) => {
                builder
                    .status(http::Status::InternalServerError)
                    .sized_body(io::Cursor::new(json!(
                        {"title": "Internal server error",
                         "detail": val
                        })
                                                        .to_string()));
            }
        }

        builder.ok()
    }
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("powergrid/static/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("powergrid").join(file)).ok()
}

#[derive(Serialize)]
struct RunInfo {
    id: String,
    model: String,
    gnome: String,
    est_life: String,
    est_power: String,
}

#[get("/runs")]
fn api_runs(db: State<DB>) -> Result<String, Error> {

    let conn = db.lock().expect("DB Lock");
    let mut stmt = conn.prepare(r#"select id, data
                    from runs
                    order by json_extract(runs.data, "$.test-name"),
                             json_extract(runs.data, "$.power") ASC"#)?;

    let rows: Result<Vec<_>, rusqlite::Error> = stmt.query_map(&[], |row| {
            let js: String = row.get(1);
            let v: serde_json::Value = serde_json::from_str(&js).expect("Valid JSON in DB");

            let info = &v["system-info"];
            let hw = &info["hardware"];
            let sw = &info["software"];
            let vendor = &hw["vendor"];
            let name = if vendor != "LENOVO" {
                &hw["name"]
            } else {
                &hw["version"]
            };

            RunInfo {
                id: row.get(0),
                model: name.as_str().unwrap().to_owned(),
                gnome: sw["gnome"]["version"].as_str().unwrap().to_owned(),
                est_life: "".to_owned(),
                est_power: "".to_owned(),
            }
        })
        .unwrap()
        .collect();

    let js = serde_json::to_string(&rows.unwrap()).ok().unwrap();
    Ok(js)
}

fn main() {
    let conn = Connection::open("powergrid/powergrid.db").expect("DB opened");
    rocket::ignite()
        .manage(Mutex::new(conn))
        .mount("/api", routes![api_runs])
        .mount("/", routes![index, files])
        .launch();
}
