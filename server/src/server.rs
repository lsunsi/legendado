mod pin_authenticate;
mod pin_request;
mod subtitle;
mod subtitles;
mod upload;

pub fn init() -> Result<rocket::Rocket, rocket_cors::Error> {
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

    Ok(rocket::ignite()
        .manage(crate::env::read())
        .attach(crate::database::Connection::fairing())
        .attach(cors)
        .mount(
            "/",
            rocket::routes![
                pin_request::pin_request,
                pin_authenticate::pin_authenticate,
                upload::upload,
                subtitles::subtitles,
                subtitle::subtitle
            ],
        ))
}
