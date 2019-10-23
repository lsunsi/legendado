#[rocket_contrib::database("pgdb")]
pub struct Connection(postgres::Connection);
