lazy_static::lazy_static! {
    static ref MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());
}

pub struct TestKit {
    pub db: legendado_server::database::Connection,
    pub client: rocket::local::Client,
    _guard: std::sync::MutexGuard<'static, ()>,
}

impl Default for TestKit {
    fn default() -> TestKit {
        let guard = MUTEX.lock().unwrap();

        let rocket = legendado_server::server::init().unwrap();
        let db = legendado_server::database::Connection::get_one(&rocket).unwrap();
        let client = rocket::local::Client::new(rocket).unwrap();

        TestKit {
            client: client,
            _guard: guard,
            db: db,
        }
    }
}

impl Drop for TestKit {
    fn drop(&mut self) {
        let rows = self
            .db
            .query(
                "
                SELECT tablename
                FROM  pg_catalog.pg_tables
                WHERE schemaname = 'public' and tablename not like '\\_%';
        ",
                &[],
            )
            .unwrap();

        for row in rows.into_iter() {
            let table_name: String = row.get(0);
            self.db
                .execute(&format!("TRUNCATE {} CASCADE", table_name), &[])
                .unwrap();
        }
    }
}
