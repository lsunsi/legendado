use rocket_contrib::json::Json;

#[derive(serde::Serialize)]
pub struct SubtitleForDownload {
    name: String,
    mime: String,
    content: Vec<u8>,
}

#[rocket::get("/subtitles/<id>")]
pub fn subtitle(
    conn: crate::database::Connection,
    user: crate::user_guard::User,
    id: i32,
) -> Json<SubtitleForDownload> {
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
