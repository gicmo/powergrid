#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate clap;

extern crate rocket;
extern crate rusqlite;

#[macro_use]
extern crate rocket_contrib;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use clap::{App, Arg, ArgMatches, SubCommand};

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

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Error {
        match error {
            _ => Error::Internal("JS error".into()),
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

            let power = format!("{:.2}", v["power"].as_f64().unwrap_or(std::f64::NAN));
            let life = v["estimated-life"]
                .as_u64()
                .map(|x| format!("{}:{}", x / 3600, x % 3600 / 60))
                .unwrap_or("N/A".to_owned());

            RunInfo {
                id: row.get(0),
                model: name.as_str().unwrap_or("N/A").to_owned(),
                gnome: sw["gnome"]["version"].as_str().unwrap_or("N/A").to_owned(),
                est_life: life,
                est_power: power,
            }
        })?
        .collect();

    rows.map_err(Error::from)
        .and_then(|ref r| serde_json::to_string(r).map_err(Error::from))
}

fn setupdb(local: &ArgMatches) -> Result<(), i32> {
    println!("🔧  Initializing Database");

    let schema_file = local.value_of("schema").unwrap_or("powergrid/schema.sql");
    println!("    => schema: {}", schema_file);

    let mut file = std::fs::File::open(schema_file)
        .map_err(|e| {
            println!("Could not open schema file: {}", e);
            1
        })?;

    let mut sql = String::new();

    file.read_to_string(&mut sql)
        .map_err(|e| {
            println!("Could not read schema file: {}", e);
            2
        })?;

    //database arg has a default value, so unwrap is safe
    let db_path = local.value_of("database").unwrap();
    println!("    => database: {}", db_path);

    let conn = Connection::open(db_path).expect("DB opened");

    conn.execute_batch(&sql)
        .map_err(|e| {
            println!("Count not create schema: {}", e);
            3
        })?;

    println!("    => DONE");
    Ok(())
}

fn main() {
    let app = App::new("powergrid")
        .version(crate_version!())
        .author(crate_authors!())
        .args(&[Arg::with_name("database")
                    .long("database")
                    .global(true)
                    .default_value("powergrid/powergrid.db")])
        .subcommand(SubCommand::with_name("setupdb")
                        .about("Initialize the database")
                        .args(&[Arg::with_name("schema").long("schema")]));

    let matches = app.clone().get_matches();

    let res = match matches.subcommand() {
        ("setupdb", Some(submatches)) => setupdb(&submatches),
        _ => Ok(()),
    };

    if let Err(i) = res {
        std::process::exit(i)
    }

    let db_path = matches.value_of("database").unwrap();
    let conn = Connection::open(db_path).expect("DB opened");

    rocket::ignite()
        .manage(Mutex::new(conn))
        .mount("/api", routes![api_runs])
        .mount("/", routes![index, files])
        .launch();
}
