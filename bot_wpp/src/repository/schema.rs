use serde::{Serialize, Deserialize};
use uuid::Uuid;
use strum_macros::{EnumString, Display};

#[derive(Serialize, EnumString, Display, Eq, PartialEq, Debug, Deserialize)]
pub enum DriverState {
    Online,
    Offline
}

#[derive(Serialize, Deserialize)]
pub struct CreateDriverSchema {
    pub driver_uuid: Uuid,
    pub name: String,
    pub online: bool,
    pub phone_number: String,
}
