pub struct Env {
    pub jwt_secret_key: String,
}

fn read_env_var(key: &str) -> String {
    std::env::var(key).expect(&format!("{} env var missing", key))
}

pub fn read() -> Env {
    Env {
        jwt_secret_key: read_env_var("JWT_SECRET_KEY"),
    }
}
