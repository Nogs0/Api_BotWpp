use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DriverModel {
    pub id: Uuid,
    pub name: String,
    pub online: bool,
    pub phone_number: String,
}
