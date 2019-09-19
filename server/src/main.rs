#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate jsonwebtoken as jwt;
extern crate postgres;

use jsonwebtoken::Validation;
use rocket::http::Status;
use rocket::request;
use rocket::Outcome;
use rocket_contrib::json::Json;
use rocket_cors;
use serde;
use std::io::Read;
use std::time::SystemTime;

const JWT_SECRET_KEY: &'static str = "puK9gTHNWhvP4vqbyP3hgiHacMfAcdH0";

#[database("pgdb")]
struct DatabaseConnection(postgres::Connection);

struct User {
    id: i32,
}

#[derive(Debug)]
enum AuthorizationError {
    MissingToken,
    InvalidToken,
}

impl<'a, 'r> request::FromRequest<'a, 'r> for User {
    type Error = AuthorizationError;

    fn from_request(request: &'a request::Request<'r>) -> request::Outcome<Self, Self::Error> {
        let authorization = request.headers().get_one("Authorization");

        let value = match authorization {
            Some(value) => value,
            None => {
                return Outcome::Failure((Status::BadRequest, AuthorizationError::MissingToken))
            }
        };

        let parts: Vec<&str> = value.split_whitespace().collect();

        let token = match &parts[..] {
            ["Bearer", token] => token,
            _ => return Outcome::Failure((Status::BadRequest, AuthorizationError::InvalidToken)),
        };

        let data = jwt::decode::<LoginClaims>(
            &token,
            JWT_SECRET_KEY.as_ref(),
            &Validation {
                validate_exp: false,
                ..Validation::default()
            },
        );

        match data {
            Ok(data) => rocket::Outcome::Success(User {
                id: data.claims.sub,
            }),
            Err(_) => {
                rocket::Outcome::Failure((Status::BadRequest, AuthorizationError::InvalidToken))
            }
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct LoginClaims {
    sub: i32,
    iat: u128,
}

#[post("/login", data = "<data>")]
fn login(conn: DatabaseConnection, data: Json<String>) -> String {
    let email = data.into_inner();

    let rows = conn
        .query("SELECT 1 FROM users WHERE email = $1 LIMIT 1", &[&email])
        .unwrap();

    if rows.is_empty() {
        conn.execute("INSERT INTO users (email) VALUES ($1)", &[&email])
            .unwrap();
    }

    let rows = conn
        .query("SELECT id FROM users WHERE email = $1 LIMIT 1", &[&email])
        .unwrap();

    jwt::encode(
        &jwt::Header::default(),
        &LoginClaims {
            sub: rows.get(0).get(0),
            iat: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis(),
        },
        JWT_SECRET_KEY.as_ref(),
    )
    .unwrap()
}

#[post("/upload?<name>&<mime>", data = "<data>")]
fn upload(conn: DatabaseConnection, user: User, name: String, mime: String, data: rocket::Data) {
    let mut bytes = vec![];

    data.open().read_to_end(&mut bytes).unwrap();

    let sql = "INSERT INTO subtitles (user_id, raw_name, mime, content) VALUES ($1, $2, $3, $4)";
    conn.execute(sql, &[&user.id, &name, &mime, &bytes])
        .unwrap();
}

#[derive(serde::Serialize)]
struct SubtitleForList {
    id: i32,
    name: String,
}

#[get("/subtitles")]
fn subtitles(conn: DatabaseConnection) -> Json<Vec<SubtitleForList>> {
    Json(
        conn.query("SELECT id, raw_name FROM subtitles", &[])
            .unwrap()
            .into_iter()
            .map(|row| SubtitleForList {
                id: row.get(0),
                name: row.get(1),
            })
            .collect(),
    )
}

#[derive(serde::Serialize)]
struct SubtitleForDownload {
    name: String,
    mime: String,
    content: Vec<u8>,
}

#[get("/subtitles/<id>")]
fn subtitle(conn: DatabaseConnection, user: User, id: i32) -> Json<SubtitleForDownload> {
    let sql = "INSERT INTO downloads (subtitle_id, user_id) VALUES ($1, $2)";
    conn.execute(sql, &[&id, &user.id]).unwrap();

    let sql = "SELECT raw_name, mime, content FROM subtitles where id = $1 LIMIT 1";
    let rows = conn.query(sql, &[&id]).unwrap();
    let row = rows.get(0);

    Json(SubtitleForDownload {
        name: row.get(0),
        mime: row.get(1),
        content: row.get(2),
    })
}

fn main() -> Result<(), rocket_cors::Error> {
    let cors = rocket_cors::CorsOptions {
        allowed_headers: rocket_cors::AllowedHeaders::all(),
        allowed_origins: rocket_cors::AllowedOrigins::some_exact(&["http://localhost:3000"]),
        allowed_methods: vec![rocket::http::Method::Get, rocket::http::Method::Post]
            .into_iter()
            .map(From::from)
            .collect(),
        ..Default::default()
    }
    .to_cors()?;

    rocket::ignite()
        .attach(DatabaseConnection::fairing())
        .attach(cors)
        .mount("/", routes![login, upload, subtitles, subtitle])
        .launch();

    Ok(())
}
