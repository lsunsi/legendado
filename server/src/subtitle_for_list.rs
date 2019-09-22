use crate::database::Connection;
use postgres;
use serde::Serialize;

#[derive(Serialize)]
struct SubtitleForListFeedback {
    key: String,
    count: i64,
    voted: bool,
}

#[derive(Serialize)]
pub struct SubtitleForList {
    id: i32,
    name: String,
    downloads_count: i64,
    feedbacks: Vec<SubtitleForListFeedback>,
}

pub fn fetch_all(conn: Connection, user_id: i32) -> Result<Vec<SubtitleForList>, postgres::Error> {
    let sub_sql = "
        SELECT s.id, s.raw_name, SUM(COALESCE(d.id, 0))
        FROM subtitles s
        LEFT JOIN downloads d ON d.subtitle_id = s.id
        GROUP BY s.id, s.raw_name
    ";

    let feedbacks_sql = "
    	SELECT s.id, f.key, count(*)
    	FROM subtitles s
        JOIN feedbacks f ON f.subtitle_id = s.id
        GROUP BY s.id, f.key
    ";

    let user_feedback_sql = "
        SELECT s.id, f.key
        FROM subtitles s
        JOIN feedbacks f ON f.subtitle_id = s.id AND f.user_id = $1
    ";

    let sub_rows = conn.query(sub_sql, &[])?;
    let feedbacks_rows = conn.query(feedbacks_sql, &[])?;
    let user_feedback_rows = conn.query(user_feedback_sql, &[&user_id])?;

    Ok(sub_rows
        .into_iter()
        .map(|row| {
            let sub_id = row.get(0);

            let feedbacks = feedbacks_rows
                .into_iter()
                .filter(|feedbacks_row| {
                    let feedbacks_sub_id: i32 = feedbacks_row.get(0);
                    sub_id == feedbacks_sub_id
                })
                .map(|feedbacks_row| {
                    let feedbacks_key = feedbacks_row.get(1);

                    let voted = user_feedback_rows.into_iter().find(|feedback_row| {
                        let feedback_sub_id: i32 = feedback_row.get(0);
                        let feedback_key: String = feedback_row.get(1);

                        feedback_sub_id == sub_id && feedbacks_key == feedback_key
                    });

                    SubtitleForListFeedback {
                        key: feedbacks_key,
                        count: feedbacks_row.get(2),
                        voted: voted.is_some(),
                    }
                });

            SubtitleForList {
                id: sub_id,
                name: row.get(1),
                downloads_count: row.get(2),
                feedbacks: feedbacks.collect(),
            }
        })
        .collect())
}
