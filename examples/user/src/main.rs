use ensemble::{types::DateTime, Model};
use std::env;

#[derive(Debug, Model)]
pub struct User {
    #[model(increments)]
    pub id: u64,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[tokio::main]
async fn main() {
    ensemble::setup(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("Failed to set up database pool.");

    let users = User::all().await.unwrap();

    dbg!(users);
}
