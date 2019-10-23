use rocket_contrib::json::Json;

#[rocket::post("/pin/request", data = "<email>")]
pub fn pin_request(conn: crate::database::Connection, email: Json<String>) {
    let pin = crate::auth::request_pin(&conn, &email).unwrap();
    println!("{}", pin);
}
