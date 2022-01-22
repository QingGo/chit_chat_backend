use crate::model::{
    chat::SingleChat,
    room::Room,
    user::{self, User},
};
use actix_session::Session;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum ErrorCode {
    Success = 0,
    RoomExisted = 10001,
    RoomNotExisted = 10002,
    LoginError = 10003,
    LogoutError = 10004,
    LoginNotValid = 10005,
    SendChatError = 10006,
    GetChatListError = 10007,
}

impl Default for ErrorCode {
    fn default() -> Self {
        ErrorCode::Success
    }
}

#[derive(Default, Serialize, Deserialize)]
struct BaseResponseBody<T> {
    code: ErrorCode,
    message: String,
    data: T,
}

pub fn gene_success_common_response(message: String) -> HttpResponse {
    HttpResponse::Ok().json(BaseResponseBody::<Option<String>> {
        code: ErrorCode::Success,
        message,
        data: None,
    })
}

pub fn gene_success_chat_list_response(chat_list: Vec<SingleChat>) -> HttpResponse {
    HttpResponse::Ok().json(BaseResponseBody::<Vec<SingleChat>> {
        code: ErrorCode::Success,
        message: "Get chat list successfully".to_string(),
        data: chat_list,
    })
}

pub fn gene_success_room_response(message: String, room: &Room) -> HttpResponse {
    HttpResponse::Ok().json(BaseResponseBody::<&Room> {
        code: ErrorCode::Success,
        message,
        data: room,
    })
}

pub fn gene_success_rooms_response(rooms: Vec<Room>) -> HttpResponse {
    HttpResponse::Ok().json(BaseResponseBody::<Vec<Room>> {
        code: ErrorCode::Success,
        message: "success".to_string(),
        data: rooms,
    })
}

pub fn gene_success_user_response(message: String, user: &User) -> HttpResponse {
    HttpResponse::Ok().json(BaseResponseBody::<&User> {
        code: ErrorCode::Success,
        message,
        data: user,
    })
}

pub fn gene_error_response(error_code: ErrorCode, message: String) -> HttpResponse {
    HttpResponse::Ok().json(BaseResponseBody::<Option<String>> {
        code: error_code,
        message,
        data: None,
    })
}

pub fn check_user_token(session: &Session) -> Result<User, String> {
    let old_user = session
        .get::<user::UserWithToken>("user")
        .map_err(|e| e.to_string())?
        .ok_or("No user field in cookies")?;
    let token = user::calc_user_token(old_user.user().id(), old_user.user().ip());
    if token != *old_user.token() {
        return Err("User token is not match".into());
    };
    Ok(old_user.user().clone())
}
