use crate::{repository::model::DriverModel, AppState};
use actix_web::{
    post,
    web::Data,
    HttpResponse, Responder,
};
use serde_json::json;

#[post("/message")]
pub async fn send_message(data: Data<AppState>) -> impl Responder {
    match sqlx::query_as!(DriverModel, "SELECT * FROM drivers WHERE online")
        .fetch_all(&data.db)
        .await
    {
        Ok(drivers) => {
            let mut message = String::from("Olá, tudo bem? Espero que sim!\nEstou indisponível no momento! 😞");
            if drivers.len() > 0{

                message += "\nMas esses motoristas estão de prontidão para lhe atender:\n";
                
                for d in drivers {
                    message += &*format!("🔵{}\n\tTelefone: {}\n", d.name, d.phone_number);
                }
            }

            let response = json!({
                "replies":[
                    {
                        "message": message
                    }
                ]
            });

            return HttpResponse::Ok().json(response);
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": format!("{error}")
            }));
        }
    }
}

