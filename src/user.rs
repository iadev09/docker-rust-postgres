use actix_web::web::Json;
use serde::{Deserialize, Serialize};
use tokio_postgres::{Error, GenericClient, Row};
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_pg_mapper::FromTokioPostgresRow;


#[derive(Debug, Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "users")]
pub struct User {
    pub id: i32,
    pub login: String,
}

impl From<Row> for User {
    fn from(row: Row) -> Self {
        Self {
            id: row.get(0),
            login: row.get(1),
        }
    }
}

impl User {
    pub async fn all<C: GenericClient>(client: &C) -> Result<Vec<User>, Error> {
        let _stmt = include_str!("../queries/user_list.sql");
        let stmt = client.prepare(&_stmt).await?;

        let rows = client.query(&stmt, &[]).await?;

        Ok(rows.into_iter().map(User::from).collect())
    }

    pub async fn create<C: GenericClient>(client: &C, user: &Json<User>) -> Result<User, Error> {
        let _stmt = include_str!("../queries/user_create.sql");
        let _stmt = _stmt.replace("$table_fields", &User::sql_table_fields());
        let stmt = client.prepare(&_stmt).await?;

        let rows = client.query(&stmt, &[&user.login]).await?;

        Ok(rows.into_iter().map(User::from).next().unwrap())
    }
}
