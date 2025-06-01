use axum::response::IntoResponse;
use axum::routing::post;
use axum::{extract::Json as JsonExtractor, Json,
           Router, };
use ollama_rs::generation::chat::request::ChatMessageRequest;
use ollama_rs::generation::chat::ChatMessage;
use ollama_rs::Ollama;
use serde::Deserialize;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main()->Result<(),Box<dyn std::error::Error>> {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    let addr = SocketAddr::from(([0,0,0,0], 3000));
    let  app = Router::new().route("/send",post( |Json(payload): JsonExtractor<ChatRequestBody>| async move  {

        let model ="afaina".to_string();

        let mut history = vec![];
        let res =  Ollama::default().send_chat_messages_with_history(
            &mut history,
            ChatMessageRequest::new(
                model,
                vec![ChatMessage::user(payload.message)]
            )
        ).await;

        match res {
            Ok(response)=>(axum::http::StatusCode::OK, Json(response)).into_response(),
            Err(e )=>(axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }

    })).layer(cors);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
#[derive(Deserialize)]
struct ChatRequestBody {
    message: String,
}
