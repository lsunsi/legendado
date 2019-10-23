use rocket::response::status;
use rocket_contrib::json::Json;

#[derive(serde::Deserialize)]
pub struct PinAuthenticationBody {
    email: String,
    pin: String,
}

#[rocket::post("/pin/authenticate", data = "<data>")]
pub fn pin_authenticate(
    data: Json<PinAuthenticationBody>,
    conn: crate::database::Connection,
    env: rocket::State<crate::env::Env>,
) -> Result<String, status::BadRequest<Json<crate::auth::PinAuthenticationError>>> {
    match crate::auth::authenticate_pin(&conn, &data.email, &data.pin) {
        Err(err) => Err(status::BadRequest(Some(Json(err)))),
        Ok(claims) => Ok(jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            env.jwt_secret_key.as_ref(),
        )
        .unwrap()),
    }
}
