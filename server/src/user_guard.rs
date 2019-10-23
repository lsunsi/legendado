use rocket::http::Status;

pub struct User {
    pub id: i32,
}

#[derive(Debug)]
pub enum AuthorizationError {
    MissingToken,
    InvalidToken,
}

impl<'a, 'r> rocket::request::FromRequest<'a, 'r> for User {
    type Error = AuthorizationError;

    fn from_request(
        request: &'a rocket::request::Request<'r>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        let env = request.guard::<rocket::State<crate::env::Env>>().unwrap();
        let authorization = request.headers().get_one("Authorization");

        let value = match authorization {
            Some(value) => value,
            None => {
                return rocket::Outcome::Failure((
                    Status::BadRequest,
                    AuthorizationError::MissingToken,
                ))
            }
        };

        let parts: Vec<&str> = value.split_whitespace().collect();

        let token = match &parts[..] {
            ["Bearer", token] => token,
            _ => {
                return rocket::Outcome::Failure((
                    Status::BadRequest,
                    AuthorizationError::InvalidToken,
                ))
            }
        };

        let data = jsonwebtoken::decode::<crate::auth::LoginClaims>(
            &token,
            env.jwt_secret_key.as_ref(),
            &jsonwebtoken::Validation {
                validate_exp: false,
                ..jsonwebtoken::Validation::default()
            },
        );

        match data {
            Ok(data) => rocket::Outcome::Success(User {
                id: data.claims.sub,
            }),
            Err(_) => {
                rocket::Outcome::Failure((Status::BadRequest, AuthorizationError::InvalidToken))
            }
        }
    }
}
