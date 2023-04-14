use std::{env, sync::Arc};

use actix_web::{middleware::Logger, rt::spawn, web, App, HttpResponse, HttpServer, Responder};

use line_bot_sdk::{
    extractor::CustomHeader,
    models::message::MessageObject,
    models::webhook_event,
    models::{
        message::text::TextMessage,
        webhook_event::{Event, Message, Root, Text},
    },
    Client,
};
use log::info;

use serde::{Deserialize, Serialize};

pub struct AppState {
    line_client: Arc<Client>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ReplyMessage {
    reply_token: String,
    messages: Vec<MessageObject>,
}

pub async fn handler(
    context: String,
    custom_header: CustomHeader,
    app_state: web::Data<AppState>,
) -> impl Responder {
    info!("Request body: {}", context);

    let client = Arc::clone(&app_state.line_client);

    let signature = get_signature_from_header(&custom_header);

    verify_signature(&client, signature, &context);

    let webhook_event = get_webhook_event(&context);

    spawn(async move { webhook_handler(&webhook_event, &client).await });

    HttpResponse::Ok().body("")
}

fn get_signature_from_header(custom_header: &CustomHeader) -> &str {
    &custom_header.x_line_signature
}

fn verify_signature(client: &Client, signature: &str, context: &str) {
    client.verify_signature(signature, context).unwrap()
}

fn get_webhook_event(context: &str) -> Root {
    serde_json::from_str(context).unwrap()
}

async fn webhook_handler(context: &webhook_event::Root, client: &Client) -> HttpResponse {
    for event in &context.events {
        reply(event, client).await;
    }
    HttpResponse::Ok().json("Ok")
}

fn get_text_message(event: &Event) -> Option<&Text> {
    match &event.message {
        Some(Message::Text(message)) => Some(message),
        _ => None,
    }
}

fn create_text_message(text: &str) -> MessageObject {
    MessageObject::Text(TextMessage::builder().text(text).build())
}

async fn reply(event: &Event, client: &Client) {
    let reply_token = event.reply_token.clone().unwrap();

    let messages = match get_text_message(event) {
        Some(text) => vec![create_text_message(&text.text)],
        None => vec![create_text_message(
            "Events other than text messages are not supported",
        )],
    };

    client.reply(&reply_token, messages, None).await.unwrap();
}

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.route("/webhook", web::post().to(handler));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .configure(router)
            .app_data(web::Data::new(AppState {
                line_client: Arc::new(Client::new(
                    env::var("CHANNEL_ACCESS_TOKEN").unwrap(),
                    env::var("CHANNEL_SECRET").unwrap(),
                )),
            }))
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
