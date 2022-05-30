use chrono::{Date, Local};
use uuid::Uuid;

#[derive(Debug)]
pub struct User {
    pub id: Uuid,

    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,

    pub created_at: Date<Local>,
    pub updated_at: Option<Date<Local>>,
    pub deleted_at: Option<Date<Local>>,
}
