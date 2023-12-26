use crate::api::{driver::*, message::*};
use actix_web::web::{scope, ServiceConfig};
use serde::*;

#[derive(Serialize, Deserialize)]
pub struct AppJson {
    pub appPackageName: String,
    pub messengerPackageName: String,
    pub query: AppDriverInfo,
}

#[derive(Serialize, Deserialize)]
pub struct AppDriverInfo {
    pub sender: String,
    pub message: String,
    pub isGroup: bool,
    pub groupParticipant: String,
    pub ruleId: u8,
    pub isTestMessage: bool,
}

pub fn config(conf: &mut ServiceConfig) {
    let scope = scope("/api")
        .service(create_driver)
        .service(update_driver)
        .service(send_message);

    conf.service(scope);
}
