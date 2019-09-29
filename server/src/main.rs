#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate chrono;
extern crate jsonwebtoken as jwt;
extern crate postgres;
extern crate rand;
extern crate rocket_cors;
extern crate serde;

use jsonwebtoken::Validation;
use rocket::http::Status;
use rocket::request;
use rocket::response::status;
use rocket::Outcome;
use rocket_contrib::json::Json;
use std::io::Read;
use rocket::State;

mod env;
use env::Env;

mod database;
use database::Connection;

mod auth;
use auth::LoginClaims;
use auth::PinAuthenticationError;

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
        let env = request.guard::<State<Env>>().unwrap();
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
            env.jwt_secret_key.as_ref(),
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

#[post("/pin/request", data = "<email>")]
fn pin_request(conn: Connection, email: Json<String>) {
    let pin = auth::request_pin(&conn, &email).unwrap();
    println!("{}", pin);
}

#[derive(serde::Deserialize)]
struct PinAuthenticationBody {
    email: String,
    pin: String,
}

#[post("/pin/authenticate", data = "<data>")]
fn pin_authenticate(
    data: Json<PinAuthenticationBody>,
    conn: Connection,
    env: State<Env>,
) -> Result<String, status::BadRequest<Json<PinAuthenticationError>>> {
    match auth::authenticate_pin(&conn, &data.email, &data.pin) {
        Err(err) => Err(status::BadRequest(Some(Json(err)))),
        Ok(claims) => {
            Ok(jwt::encode(&jwt::Header::default(), &claims, env.jwt_secret_key.as_ref()).unwrap())
        }
    }
}

#[post("/upload?<name>&<mime>", data = "<data>")]
fn upload(conn: Connection, user: User, name: String, mime: String, data: rocket::Data) {
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
    downloads_count: i64,
}

#[get("/subtitles")]
fn subtitles(conn: Connection) -> Json<Vec<SubtitleForList>> {
    let sql = "
        SELECT s.id, s.raw_name, (
            SELECT count(distinct(d.user_id)) FROM downloads d WHERE subtitle_id = s.id
        )
        FROM subtitles s
    ";

    Json(
        conn.query(sql, &[])
            .unwrap()
            .into_iter()
            .map(|row| SubtitleForList {
                id: row.get(0),
                name: row.get(1),
                downloads_count: row.get(2),
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
fn subtitle(conn: Connection, user: User, id: i32) -> Json<SubtitleForDownload> {
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
        .manage(env::read())
        .attach(Connection::fairing())
        .attach(cors)
        .mount(
            "/",
            routes![pin_request, pin_authenticate, upload, subtitles, subtitle],
        )
        .launch();

    Ok(())
}
