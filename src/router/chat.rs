use super::utils::{gene_error_response, ErrorCode};
use crate::model::chat;
use crate::model::room::new_room;
use crate::router::utils::{
    check_user_token, gene_success_chat_list_response, gene_success_common_response,
};
use crate::utils::format_and_debug_msg;
use actix_session::Session;
use actix_web::{get, post, web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct ChatRequest {
    chat: String,
}

#[post("chat/{room_id}")]
async fn send_chat(
    session: Session,
    req: web::Json<ChatRequest>,
    web::Path(room_id): web::Path<String>,
) -> HttpResponse {
    let user = check_user_token(&session);
    if let Err(e) = user {
        let message = format_and_debug_msg!("User not login: {}", e);
        return gene_error_response(ErrorCode::LoginNotValid, message);
    }
    let user = user.unwrap();
    let room = new_room(&room_id);
    let result = chat::send_chat(room, user, &req.chat);
    if let Err(e) = result {
        let message = format_and_debug_msg!("Send chat error: {}", e);
        return gene_error_response(ErrorCode::SendChatError, message);
    }
    gene_success_common_response("Send chat successfully".to_string())
}

#[get("chat/{room_id}")]
async fn get_chat_list(session: Session, web::Path(room_id): web::Path<String>) -> HttpResponse {
    let user = check_user_token(&session);
    if let Err(e) = user {
        let message = format_and_debug_msg!("User not login: {}", e);
        return gene_error_response(ErrorCode::LoginNotValid, message);
    }
    let room = new_room(&room_id);
    let chat_list = chat::get_chat_list(&room);
    if let Err(e) = chat_list {
        let message = format_and_debug_msg!("Get chat list error: {}", e);
        return gene_error_response(ErrorCode::GetChatListError, message);
    }
    gene_success_chat_list_response(chat_list.unwrap())
}
