mod test_kit;

#[test]
fn test_user_successful_upload() {
    let kit = test_kit::TestKit::default();

    assert_eq!(
        kit.client
            .post("/pin/request")
            .body("\"meira@mineracao.com\"")
            .dispatch()
            .status(),
        rocket::http::Status::Ok
    );

    let pin: String = kit
        .db
        .query("SELECT pin FROM authentication_pins LIMIT 1", &[])
        .unwrap()
        .get(0)
        .get(0);

    let mut response = kit
        .client
        .post("/pin/authenticate")
        .body(format!(
            "{{\"email\":\"meira@mineracao.com\",\"pin\":\"{}\"}}",
            pin
        ))
        .dispatch();

    assert_eq!(response.status(), rocket::http::Status::Ok);
    let token = response.body_string().unwrap();

    let sql = "SELECT * FROM subtitles";
    assert_eq!(kit.db.query(sql, &[]).unwrap().len(), 0);

    let response = kit
        .client
        .post("/upload?name=OsVingaceiros&mime=text/legenda")
        .header(rocket::http::Header::new(
            "Authorization",
            format!("Bearer {}", token),
        ))
        .body("Toninho: Eu sou o Homem de Ferro.")
        .dispatch();

    assert_eq!(response.status(), rocket::http::Status::Ok);

    let rows = kit.db.query("SELECT id FROM users", &[]).unwrap();
    assert_eq!(rows.len(), 1);
    let user_id: i32 = rows.get(0).get(0);

    let rows = kit
        .db
        .query("SELECT raw_name, mime, user_id, content FROM subtitles", &[])
        .unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows.get(0).get::<usize, String>(0), "OsVingaceiros");
    assert_eq!(rows.get(0).get::<usize, String>(1), "text/legenda");
    assert_eq!(rows.get(0).get::<usize, i32>(2), user_id);
    assert_eq!(
        std::str::from_utf8(&rows.get(0).get::<usize, Vec<u8>>(3)).unwrap(),
        "Toninho: Eu sou o Homem de Ferro."
    );
}
