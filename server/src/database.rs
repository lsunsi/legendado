use postgres;

#[database("pgdb")]
pub struct Connection(postgres::Connection);
