use chrono::{Date, Local};
use uuid::Uuid;

use crate::User;

#[derive(Debug)]
pub struct Book {
    pub id: Uuid,

    pub title: String,
    pub author: User,
    pub summary: String,
    pub pages: u32,

    pub created_at: Date<Local>,
    pub updated_at: Option<Date<Local>>,
    pub deleted_at: Option<Date<Local>>,
}
