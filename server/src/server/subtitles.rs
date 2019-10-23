use rocket_contrib::json::Json;

#[derive(serde::Serialize)]
pub struct SubtitleForList {
    id: i32,
    name: String,
    downloads_count: i64,
}

#[rocket::get("/subtitles")]
pub fn subtitles(conn: crate::database::Connection) -> Json<Vec<SubtitleForList>> {
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
