mod test_kit;

fn authenticate_email(kit: &test_kit::TestKit, email: &str) -> String {
    assert_eq!(
        kit.client
            .post("/pin/request")
            .body(format!("\"{}\"", email))
            .dispatch()
            .status(),
        rocket::http::Status::Ok
    );

    let pin: String = kit
        .db
        .query(
            "
            SELECT pin
            FROM authentication_pins p
            JOIN users u ON u.id = p.user_id
            WHERE u.email = $1
            ORDER BY p.created_at DESC
            LIMIT 1
        ",
            &[&email],
        )
        .unwrap()
        .get(0)
        .get(0);

    let mut response = kit
        .client
        .post("/pin/authenticate")
        .body(format!("{{\"email\":\"{}\",\"pin\":\"{}\"}}", email, pin))
        .dispatch();

    assert_eq!(response.status(), rocket::http::Status::Ok);
    response.body_string().unwrap()
}

#[test]
fn test_user_successful_upload() {
    let kit = test_kit::TestKit::default();
    let token = authenticate_email(&kit, "meira@mineracao.com");

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
        .query(
            "SELECT raw_name, mime, user_id, content FROM subtitles",
            &[],
        )
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

#[test]
fn test_user_uploads_list() {
    let kit = test_kit::TestKit::default();
    let uploader_token = authenticate_email(&kit, "meira@mineracao.com");
    let downloader_a_token = authenticate_email(&kit, "mariza@modal.com");
    let downloader_b_token = authenticate_email(&kit, "gisele@bramstoker.com");

    assert_eq!(
        kit.client
            .post("/upload?name=OsVingaceiros&mime=text/legenda")
            .header(rocket::http::Header::new(
                "Authorization",
                format!("Bearer {}", uploader_token),
            ))
            .body("Toninho: Eu sou o Homem de Ferro.")
            .dispatch()
            .status(),
        rocket::http::Status::Ok
    );

    assert_eq!(
        kit.client
            .post("/upload?name=MiranhaNomade&mime=text/legenda")
            .header(rocket::http::Header::new(
                "Authorization",
                format!("Bearer {}", uploader_token),
            ))
            .body("Pedro: Sinto falta do meu tio.")
            .dispatch()
            .status(),
        rocket::http::Status::Ok
    );

    let rows = kit.db.query("SELECT * FROM downloads", &[]).unwrap();
    assert_eq!(rows.len(), 0);

    let rows = kit.db.query("SELECT id FROM subtitles ORDER BY id", &[]).unwrap();
    assert_eq!(rows.len(), 2);

    let subtitle_vingaceiros: i32 = rows.get(0).get(0);
    let subtitle_miranha: i32 = rows.get(1).get(0);

    assert_eq!(
        kit.client
            .get(format!("/subtitles/{}", subtitle_vingaceiros))
            .header(rocket::http::Header::new(
                "Authorization",
                format!("Bearer {}", downloader_a_token),
            ))
            .dispatch()
            .status(),
        rocket::http::Status::Ok
    );

    assert_eq!(
        kit.client
            .get(format!("/subtitles/{}", subtitle_miranha))
            .header(rocket::http::Header::new(
                "Authorization",
                format!("Bearer {}", downloader_a_token),
            ))
            .dispatch()
            .status(),
        rocket::http::Status::Ok
    );

    assert_eq!(
        kit.client
            .get(format!("/subtitles/{}", subtitle_miranha))
            .header(rocket::http::Header::new(
                "Authorization",
                format!("Bearer {}", downloader_b_token),
            ))
            .dispatch()
            .status(),
        rocket::http::Status::Ok
    );

    let response = kit.client
            .get("/uploads")
            .header(rocket::http::Header::new(
                "Authorization",
                format!("Bearer {}", downloader_b_token),
            ))
            .dispatch();

    assert_eq!(response.status(), rocket::http::Status::Ok);
}
