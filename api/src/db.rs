use sea_orm::{Database, DatabaseConnection, DbErr};

pub async fn conn(url: String) -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(url).await?;
    Ok(db)
}
