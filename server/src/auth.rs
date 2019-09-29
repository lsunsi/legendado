use crate::database::Connection;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

fn fetch_user_id_by_email(conn: &Connection, email: &str) -> Result<Option<i32>, postgres::Error> {
    let rows = conn.query("SELECT id FROM users WHERE email = $1 LIMIT 1", &[&email])?;
    Ok(rows.into_iter().next().map(|row| row.get(0)))
}

fn generate_pin() -> String {
    let mut rng = thread_rng();
    let mut pin = String::from("");

    for _ in 0..4 {
        pin = format!("{}{}", pin, rng.gen_range(0, 10));
    }

    pin
}

pub fn request_pin(conn: &Connection, email: &str) -> Result<String, postgres::Error> {
    let user_id = fetch_user_id_by_email(&conn, email)?;

    if user_id.is_none() {
        conn.execute("INSERT INTO users (email) VALUES ($1)", &[&email])?;
    }

    let user_id = fetch_user_id_by_email(&conn, email)?.unwrap();
    let pin = generate_pin();

    conn.execute(
        "INSERT INTO authentication_pins (user_id, pin, created_at) VALUES ($1, $2, $3)",
        &[&user_id, &pin, &chrono::Utc::now()],
    )?;

    Ok(pin)
}

#[derive(Serialize, Debug)]
pub enum PinAuthenticationError {
    InexistentUser,
    InexistentPin,
    ExhaustedAttemps,
    MismatchedPins,
    ExpiredPin,
}

#[derive(Serialize, Deserialize)]
pub struct LoginClaims {
    iat: chrono::DateTime<chrono::offset::Utc>,
    pub sub: i32,
}

pub fn authenticate_pin(
    conn: &Connection,
    email: &str,
    pin: &str,
) -> Result<LoginClaims, PinAuthenticationError> {
    let now = chrono::Utc::now();

    let user_id = match fetch_user_id_by_email(&conn, email).unwrap() {
        None => return Err(PinAuthenticationError::InexistentUser),
        Some(user_id) => user_id,
    };

    let sql =
        "INSERT INTO authentication_pin_attempts (pin, user_id, created_at) VALUES ($1, $2, $3)";
    conn.execute(sql, &[&pin, &user_id, &now]).unwrap();

    let sql = "
        SELECT
            (
                SELECT COUNT(*)
                FROM authentication_pin_attempts a
                WHERE a.user_id = p.user_id AND a.created_at > p.created_at
            ),
            (
                SELECT COUNT(*)
                FROM authentication_pin_attempts a
                WHERE a.user_id = p.user_id AND a.created_at > p.created_at AND a.pin = p.pin
            )
        FROM authentication_pins p
        WHERE user_id = $1
        ORDER BY p.created_at DESC
        LIMIT 1
    ";

    let rows = conn.query(sql, &[&user_id]).unwrap();

    let (all_attempts, correct_attempts): (i64, i64) = match rows.into_iter().next() {
        None => return Err(PinAuthenticationError::InexistentPin),
        Some(row) => (row.get(0), row.get(1)),
    };

    if all_attempts > 3 {
        Err(PinAuthenticationError::ExhaustedAttemps)
    } else if correct_attempts > 1 {
        Err(PinAuthenticationError::ExpiredPin)
    } else if correct_attempts == 0 {
        Err(PinAuthenticationError::MismatchedPins)
    } else {
        Ok(LoginClaims {
            sub: user_id,
            iat: now,
        })
    }
}
