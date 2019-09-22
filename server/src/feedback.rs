use crate::database::Connection;
use postgres;
use std::str::FromStr;
use std::string::ToString;

pub enum Feedback {
    JustWorks,
    BadSync,
}

impl FromStr for Feedback {
    type Err = &'static str;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        match raw {
            "JustWorks" => Ok(Self::JustWorks),
            "BadSync" => Ok(Self::BadSync),
            _ => Err(":("),
        }
    }
}

impl ToString for Feedback {
    fn to_string(&self) -> String {
        String::from(match self {
            Feedback::JustWorks => "JustWorks",
            Feedback::BadSync => "BadSync",
        })
    }
}

pub fn insert(
    conn: Connection,
    feedback: Feedback,
    user_id: i32,
    subtitle_id: i32,
) -> Result<(), postgres::Error> {
    let transaction = conn.transaction()?;

    let sql = "DELETE FROM feedbacks WHERE user_id = $1 AND subtitle_id = $2";
    transaction.execute(sql, &[&user_id, &subtitle_id])?;

    let sql = "INSERT INTO feedbacks (user_id, subtitle_id, key) VALUES ($1, $2, $3)";
    transaction.execute(sql, &[&user_id, &subtitle_id, &feedback.to_string()])?;

    transaction.commit()?;

    Ok(())
}
