use deadpool_postgres::Pool;
use tokio_postgres_migration::Migration;

const SCRIPTS_UP: [(&str, &str); 2] = [
    (
        "0001_create-users",
        include_str!("../migrations/0001_create-users_up.sql"),
    ),
    (
        "0002_add-root-user-to-users",
        include_str!("../migrations/0002_add-root-user-to-users_up.sql"),
    ),
];

pub async fn up(pool: &Pool) {
    let mut client = pool.get().await.expect("couldn't get postgres client");
    let migration = Migration::new("migrations".to_string());
    migration
        .up(&mut **client, &SCRIPTS_UP)
        .await
        .expect("couldn't run migrations");
}
