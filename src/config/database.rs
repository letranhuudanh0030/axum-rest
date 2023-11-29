use super::{
    error::{Error, Result},
    init::web_config,
};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
pub type Db = DatabaseConnection;

pub async fn connect_db() -> Result<Db> {
    let max_connections = if cfg!(test) { 1 } else { 5 };
    let mut opt = ConnectOptions::new(&web_config().DB_URL);
    opt.max_connections(max_connections).sqlx_logging(false);

    Database::connect(opt)
        .await
        .map_err(|ex| Error::FailToCreatePool(ex.to_string()))
}
