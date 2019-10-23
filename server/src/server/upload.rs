use std::io::Read;

#[rocket::post("/upload?<name>&<mime>", data = "<data>")]
pub fn upload(
    conn: crate::database::Connection,
    user: crate::user_guard::User,
    name: String,
    mime: String,
    data: rocket::Data,
) {
    let mut bytes = vec![];

    data.open().read_to_end(&mut bytes).unwrap();

    let sql = "INSERT INTO subtitles (user_id, raw_name, mime, content) VALUES ($1, $2, $3, $4)";
    conn.execute(sql, &[&user.id, &name, &mime, &bytes])
        .unwrap();
}
