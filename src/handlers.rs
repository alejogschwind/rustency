
use actix_web::Responder;

pub async fn get_currencies() -> impl Responder {
    format!("Hello from get currencies")
}

pub async fn get_currency_by_id() -> impl Responder {
    format!("Hello from get currency by id")
}

pub async fn add_currency() -> impl Responder {
    format!("Hello from add currency")
}

pub async fn delete_currency() -> impl Responder {
    format!("Hello from delete currency")
}