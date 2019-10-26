mod test_kit;

#[test]
fn test_new_user_authentication_success() {
    let kit = test_kit::TestKit::default();

    // starts off with no users
    let rows = kit.db.query("SELECT * FROM users", &[]).unwrap();
    assert_eq!(rows.len(), 0);

    // starts off with no pins
    let rows = kit
        .db
        .query("SELECT * FROM authentication_pins", &[])
        .unwrap();
    assert_eq!(rows.len(), 0);

    // pin request works fine
    let response = kit
        .client
        .post("/pin/request")
        .body("\"meira@mineracao.com\"")
        .dispatch();
    assert_eq!(response.status(), rocket::http::Status::Ok);

    // user with given email is created
    let sql = "SELECT id, email FROM users";
    let rows = kit.db.query(sql, &[]).unwrap();

    assert_eq!(rows.len(), 1);
    assert_eq!(rows.get(0).get::<usize, String>(1), "meira@mineracao.com");
    let user_id: i32 = rows.get(0).get(0);

    // and pin is created for that user
    let sql = "SELECT id, user_id, pin FROM authentication_pins";
    let rows = kit.db.query(sql, &[]).unwrap();

    assert_eq!(rows.len(), 1);
    assert_eq!(rows.get(0).get::<usize, i32>(1), user_id);
    let pin: String = rows.get(0).get(2);

    // although no attemps exist yet
    let sql = "SELECT * FROM authentication_pin_attempts";
    let rows = kit.db.query(sql, &[]).unwrap();
    assert_eq!(rows.len(), 0);

    // pin authentication works fine
    let mut response = kit
        .client
        .post("/pin/authenticate")
        .body(format!(
            "{{\"email\":\"meira@mineracao.com\",\"pin\":\"{}\"}}",
            pin
        ))
        .dispatch();

    assert_eq!(response.status(), rocket::http::Status::Ok);
    assert!(response.body_string().unwrap().len() > 0);
}
