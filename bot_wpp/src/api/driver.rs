use actix_web:: {
    web::{
        Data, Json
    },
    post,
    put,
    HttpResponse,
    Responder
};
use serde_json::json;
use serde::*;
use crate::{AppState, services::AppJson, repository::model::DriverModel};


#[derive(Serialize, Deserialize)]
pub struct DriverIdentifier {
    pub name: String,
    pub phone_number: String,
    pub online: bool
}

#[post("/driver/create")]
pub async fn create_driver(
    body: Json<DriverIdentifier>,
    data: Data<AppState>
) -> impl Responder {

    match sqlx::query(
        "INSERT INTO drivers (name, phone_number, online) VALUES ($1, $2, $3) RETURNING *")
        .bind(&body.name)
        .bind(&body.phone_number)
        .bind(&body.online)
        .fetch_one(&data.db)
        .await {
            Ok(_driver) => {
                let note_response = json!({
                "status": "success"
                });

                return HttpResponse::Ok().json(note_response)
            }
            Err(error) => {
                return HttpResponse::InternalServerError().json(
                    json!({
                        "status": "error",
                        "message": format!("{error}")
                    })
                )
            }
    }
}

#[post("/driver/update")]
pub async fn update_driver(
    body: Json<AppJson>,
    data: Data<AppState>
) -> impl Responder {

    match sqlx::query_as!(
        DriverModel,
        "UPDATE drivers SET online = $1 WHERE phone_number like $2 RETURNING *",
        (body.query.message.to_uppercase().trim() == "ONLINE"),
        &body.query.groupParticipant.trim())
        .fetch_one(&data.db)
        .await {
            Ok(driver) => {
                let state = if driver.online {"Online"} else {"Offline"};
                let response = json!({
                    "replies":[
                        {
                            "message": format!("Motorista {} está {}!", driver.name, state)
                        }
                    ]
                });

                return HttpResponse::Ok().json(response)
            }
            Err(error) => {
                let response = json!({
                    "replies":[
                        {
                            "message": format!("ERRO: {}!", error)
                        }
                    ]
                });

                return HttpResponse::Ok().json(response)
            }
    }
}
