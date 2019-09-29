#[database("pgdb")]
pub struct Connection(postgres::Connection);
